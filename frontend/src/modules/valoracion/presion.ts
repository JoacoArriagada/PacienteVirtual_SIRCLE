import { api, escribir } from "../../app/api.js";
import { graficaPresion, metricasPresion } from "./historico-presion.js";
import { iconoValoracion } from "./brazo.js";

export interface MedicionPresion {
  medicion_id: string;
  intento_id: string;
  valor: { sistolica: number; diastolica: number };
  unidad: string;
  momento: string;
  origen?: string;
  en_ficha: boolean;
}

interface VistaPresion {
  titulo: HTMLElement;
  estado: HTMLElement;
  escena: HTMLElement;
  controles: HTMLElement;
  instrumento: HTMLElement;
  panelHistorico: HTMLElement;
  inicial: () => void;
}

function monitor(duracion: number): string {
  const puntos = Array.from({ length: 481 }, (_, indice) => {
    const fraccion = indice / 480;
    const inflado = fraccion < 0.32 ? fraccion / 0.32 : (1 - fraccion) / 0.68;
    const amplitud = Math.exp(-(((fraccion - 0.61) / 0.2) ** 2));
    const senal = 0.13 + 0.72 * inflado + 0.09 * amplitud * Math.sin(2 * Math.PI * 32 * fraccion);
    return `${(600 * fraccion).toFixed(2)},${(94 - 83 * senal).toFixed(2)}`;
  }).join(" ");
  return `<figure class="live-monitor">
    <figcaption class="monitor-caption"><strong>Presión del brazalete</strong><span>Señal simulada</span></figcaption>
    <svg class="monitor-plot" viewBox="0 0 600 104" preserveAspectRatio="none" aria-hidden="true">
      <defs>
        <pattern id="rejilla-presion" width="30" height="20" patternUnits="userSpaceOnUse">
          <path d="M0 20V0H30" fill="none" stroke="#34536b"></path>
        </pattern>
        <clipPath id="recorte-presion"><rect data-revelar width="0" height="104"></rect></clipPath>
      </defs>
      <rect width="100%" height="100%" fill="url(#rejilla-presion)"></rect>
      <polyline points="${puntos}" clip-path="url(#recorte-presion)" fill="none" stroke="#8ce8cd"
        stroke-width="2" vector-effect="non-scaling-stroke" stroke-linejoin="round"></polyline>
    </svg>
    <div class="monitor-axis" aria-hidden="true"><span>0 s</span><span>${duracion} s</span></div>
  </figure>`;
}

export function prepararPresion(intento: string, vista: VistaPresion) {
  const { titulo, estado, escena, controles, instrumento, panelHistorico, inicial } = vista;
  const ruta = "/intentos/" + encodeURIComponent(intento) + "/mediciones";
  const duracion = 20;
  let reloj: number | undefined;
  let confirmando = false;
  let cerrada = false;
  let ultima: MedicionPresion | undefined;

  function cambiar(fase: string, nombre: string, mensaje: string): void {
    escena.dataset.phase = fase;
    titulo.textContent = "Presión arterial · " + nombre;
    estado.textContent = mensaje;
  }

  function cerrar(): boolean {
    if (confirmando) return false;
    window.clearInterval(reloj);
    cerrada = true;
    return true;
  }

  function comenzar(): void {
    cambiar("running", "En curso", "Acción en curso");
    controles.innerHTML = `
      <h2>Presión arterial en curso</h2><p>El monitor está tomando la presión.</p>
      ${metricasPresion(undefined, "—")}
      ${monitor(duracion)}
      <div class="stack tight">
        <progress max="${duracion}" value="0" aria-label="Progreso del procedimiento"></progress>
        <p class="procedure-progress muted" data-tiempo>${duracion} s restantes</p>
      </div>
      <div class="button-row">
        <button class="button" type="button" data-cancelar>Cancelar toma</button>
      </div>`;
    const progreso = controles.querySelector("progress")!;
    const segundos = controles.querySelector("[data-tiempo]")!;
    const revelado = controles.querySelector("[data-revelar]")!;
    const cancelar = controles.querySelector<HTMLButtonElement>("[data-cancelar]")!;
    const inicio = performance.now();
    const clave = crypto.randomUUID();
    cancelar.addEventListener("click", () => {
      window.clearInterval(reloj);
      inicial();
    });
    titulo.focus();
    reloj = window.setInterval(() => {
      if (!controles.isConnected) {
        window.clearInterval(reloj);
        return;
      }
      const transcurrido = Math.min(duracion, (performance.now() - inicio) / 1000);
      progreso.value = transcurrido;
      segundos.textContent = `${Math.ceil(duracion - transcurrido)} s restantes`;
      revelado.setAttribute("width", String(600 * transcurrido / duracion));
      if (transcurrido === duracion) {
        window.clearInterval(reloj);
        cancelar.disabled = true;
        void confirmar(clave);
      }
    }, 250);
  }

  async function confirmar(clave: string): Promise<void> {
    if (confirmando || cerrada) return;
    confirmando = true;
    estado.textContent = "Guardando la lectura";
    try {
      const lectura = await api<MedicionPresion>(ruta,
        escribir("POST", { instrumento: "presion", accion: "toma_completa" }, clave));
      resultado(lectura);
    } catch (error) {
      mostrarError(error, () => { void confirmar(clave); });
    } finally {
      confirmando = false;
    }
  }

  function resultado(lectura: MedicionPresion): void {
    ultima = lectura;
    panelHistorico.hidden = true;
    instrumento.hidden = false;
    cambiar("result", "Resultado", "Resultado listo");
    escena.dataset.positioned = "true";
    controles.innerHTML = `
      <h2>Resultado disponible</h2>
      <div class="stack tight">
        <p class="source-label" data-momento></p>
        ${metricasPresion(lectura)}
        <p class="state" role="status" data-registro></p>
      </div>
      <p class="small-text muted" data-origen></p>
      <div class="button-row">
        <button class="button primary" type="button" data-registrar>Registrar en ficha</button>
        <button class="button" type="button" data-nueva>Repetir toma</button>
        <button class="button" type="button" data-historico>Histórico</button>
      </div>`;
    controles.querySelector<HTMLElement>("[data-momento]")!.textContent =
      [lectura.medicion_id, lectura.momento, "Brazalete y monitor"].join(" · ");
    controles.querySelector("[data-origen]")!.textContent = lectura.origen ?? "Lectura del triaje simulado de LR.";
    const estadoRegistro = controles.querySelector<HTMLElement>("[data-registro]")!;
    const registrar = controles.querySelector<HTMLButtonElement>("[data-registrar]")!;
    const clave = crypto.randomUUID();
    function mostrarRegistro(): void {
      estadoRegistro.textContent = lectura.en_ficha ? "Registrada en ficha" : "Por registrar";
      estadoRegistro.dataset.tone = lectura.en_ficha ? "success" : "waiting";
    }
    mostrarRegistro();
    registrar.addEventListener("click", async () => {
      registrar.disabled = true;
      try {
        const acuse = await api<MedicionPresion>(
          ruta + "/" + encodeURIComponent(lectura.medicion_id) + "/ficha", escribir("PUT", {}, clave));
        lectura = acuse;
        ultima = lectura;
        mostrarRegistro();
      } catch (error) {
        estadoRegistro.textContent = error instanceof Error ? error.message : "No se guardó la lectura.";
      } finally {
        registrar.disabled = false;
      }
    });
    controles.querySelector("[data-historico]")!.addEventListener("click", historico);
    controles.querySelector("[data-nueva]")!.addEventListener("click", inicial);
    titulo.focus();
  }

  async function historico(): Promise<void> {
    cambiar("history", "Histórico", "Histórico de tomas");
    instrumento.hidden = true;
    panelHistorico.hidden = false;
    panelHistorico.innerHTML = '<p role="status">Cargando tomas…</p>';
    titulo.focus();
    try {
      const lecturas = await api<MedicionPresion[]>(ruta + "?instrumento=presion");
      pintarHistorico(lecturas);
    } catch (error) {
      mostrarError(error, historico, panelHistorico);
    }
  }

  function pintarHistorico(lecturas: MedicionPresion[]): void {
    panelHistorico.innerHTML = `
      <div class="button-row between with-back">
        <button class="button" type="button" data-regresar>${iconoValoracion("vista-volver", 20)}<span>Volver al instrumento</span></button>
        <div class="heading-copy"><h2>Histórico de presión arterial</h2><p class="muted">${lecturas.length} ${lecturas.length === 1 ? "toma completa" : "tomas completas"}</p></div>
      </div>
      <div class="tabs" role="group" aria-label="Vista del histórico">
        <button type="button" class="tab" data-grafica aria-pressed="true">Gráfica</button>
        <button type="button" class="tab" data-tabla aria-pressed="false">Tabla</button>
      </div>
      <div class="field"><label for="mostrar-tomas">Mostrar tomas</label>
        <select id="mostrar-tomas"><option value="todas">Todas</option><option value="pendientes">Sin registrar</option></select></div>
      <div data-tomas></div>
      <div class="button-row"><button type="button" class="button" data-nueva>Preparar otra toma</button></div>`;
    const grafica = panelHistorico.querySelector<HTMLButtonElement>("[data-grafica]")!;
    const tabla = panelHistorico.querySelector<HTMLButtonElement>("[data-tabla]")!;
    const cuerpo = panelHistorico.querySelector<HTMLElement>("[data-tomas]")!;
    const filtro = panelHistorico.querySelector("select")!;
    let modo: "grafica" | "tabla" = "grafica";
    panelHistorico.querySelector("[data-regresar]")!.addEventListener("click", () => {
      if (ultima) resultado(ultima);
      else inicial();
    });
    panelHistorico.querySelector("[data-nueva]")!.addEventListener("click", inicial);
    grafica.addEventListener("click", () => { modo = "grafica"; pintar(); });
    tabla.addEventListener("click", () => { modo = "tabla"; pintar(); });
    filtro.addEventListener("change", pintar);
    pintar();

    function pintar(): void {
      grafica.setAttribute("aria-pressed", String(modo === "grafica"));
      tabla.setAttribute("aria-pressed", String(modo === "tabla"));
      const visibles = filtro.value === "pendientes" ? lecturas.filter((toma) => !toma.en_ficha) : lecturas;
      if (visibles.length === 0) {
        cuerpo.innerHTML = lecturas.length
          ? '<p class="muted">Todas las tomas están registradas.</p>'
          : '<p class="muted">Aún no hay tomas.</p>';
      } else if (modo === "grafica") {
        cuerpo.innerHTML = graficaPresion(visibles);
      } else {
        cuerpo.innerHTML = `<div class="table-scroll"><table><caption>Tomas del intento</caption>
          <thead><tr><th scope="col">Toma</th><th scope="col">Momento</th><th scope="col">Sistólica (mmHg)</th>
          <th scope="col">Diastólica (mmHg)</th><th scope="col">Ficha</th><th scope="col">Acción</th></tr></thead>
          <tbody></tbody></table></div>`;
        for (const toma of visibles) {
          const fila = cuerpo.querySelector("tbody")!.insertRow();
          fila.dataset.medicionId = toma.medicion_id;
          for (const valor of [toma.medicion_id, toma.momento, toma.valor.sistolica, toma.valor.diastolica]) {
            fila.insertCell().textContent = String(valor);
          }
          const ficha = document.createElement("span");
          ficha.className = "state";
          ficha.dataset.tone = toma.en_ficha ? "success" : "waiting";
          ficha.textContent = toma.en_ficha ? "Registrada" : "Sin registrar";
          fila.insertCell().append(ficha);
          const abrir = document.createElement("button");
          abrir.className = "button small";
          abrir.type = "button";
          abrir.textContent = "Ver toma";
          abrir.addEventListener("click", () => resultado(toma));
          fila.insertCell().append(abrir);
        }
      }
    }
  }

  function mostrarError(error: unknown, reintentar: () => void, panel = controles): void {
    const mensaje = document.createElement("p");
    mensaje.setAttribute("role", "status");
    mensaje.textContent = error instanceof Error ? error.message : "No se pudo completar la operación.";
    const boton = document.createElement("button");
    boton.textContent = "Reintentar";
    boton.className = "button";
    boton.type = "button";
    boton.addEventListener("click", reintentar);
    panel.replaceChildren(mensaje, boton);
    boton.focus();
  }

  return { comenzar, historico, cerrar };
}
