import { api, escribir } from "../../app/api.js";
import { iconoValoracion, retratoLR } from "./brazo.js";

export interface Medicion {
  instrumento: string;
  medicion_id: string;
  intento_id: string;
  valor: Record<string, number>;
  unidad: string;
  origen?: string;
  momento: string;
  en_ficha: boolean;
}

export interface Instrumento {
  id: string;
  nombre: string;
  metodo: string;
  icono: string;
  imagen: string;
  preparar: string;
  iniciar: string;
  pasos: readonly string[];
  duracion: number;
  instrucciones: string;
  sensor?: { archivo: string; clase: string; paso: number | "preparacion"; x: number; y: number };
  escenaColocada?: { archivo: string; paso: number };
  conteo?: { ciclos: number; x: number; y: number };
}

export function textoMedicion(medicion: Pick<Medicion, "instrumento" | "valor" | "unidad">): string {
  const { valor, unidad } = medicion;
  if (medicion.instrumento === "presion") return `${valor.sistolica}/${valor.diastolica} ${unidad}`;
  if ("ciclos" in valor) return `${valor.frecuencia} ${unidad} · ${valor.ciclos} ciclos marcados`;
  return `${(valor.frecuencia ?? valor.lectura).toLocaleString("es-CL")} ${unidad}`;
}

export function mostrarInstrumento(main: HTMLElement, intento: string, config: Instrumento, volver: () => void): void {
  const ruta = "/intentos/" + encodeURIComponent(intento) + "/mediciones";
  main.dataset.view = "procedure";
  main.innerHTML = `
    <section class="view procedure-view" aria-labelledby="page-title">
      <div class="page-heading with-back">
        <div class="heading-back"><button class="button" type="button" data-atencion>
          ${iconoValoracion("vista-volver", 20)}<span>Volver a la atención</span>
        </button></div>
        <div class="heading-copy"><h1 id="page-title" tabindex="-1"></h1><p class="muted">${config.metodo} · Simulación</p></div>
      </div>
      <div class="procedure-grid">
        <div class="panel">
          <div class="procedure-portrait"><img class="portrait" src="${retratoLR}" alt="" width="38" height="38">
            <strong>LR</strong><span class="muted">· +00 min</span>
          </div>
          <figure class="stack tight">
            <div id="device-art" class="device-art" data-phase="initial" data-positioned="false">
              <img class="procedure-photo" src="${config.imagen}" alt="Escena clínica de LR" width="756" height="504">
              ${config.escenaColocada
                ? `<img class="procedure-photo positioned-photo" src="${config.escenaColocada.archivo}" alt="" width="756" height="504">`
                : ""}
            </div>
            <figcaption class="small-text muted">${config.metodo} · Escena de simulación</figcaption>
          </figure>
        </div>
        <section class="panel stack" aria-label="Controles del instrumento">
          <div class="procedure-state"><span>Estado del procedimiento</span><strong data-estado></strong></div>
          <div data-controles></div>
        </section>
      </div>
    </section>`;
  const escena = main.querySelector<HTMLElement>("#device-art")!;
  const foto = escena.querySelector("img")!;
  const titulo = main.querySelector<HTMLElement>("#page-title")!;
  const estado = main.querySelector<HTMLElement>("[data-estado]")!;
  const controles = main.querySelector<HTMLElement>("[data-controles]")!;
  let reloj: number | undefined;
  let confirmando = false;
  let ciclos = 0;

  const ubicacion = config.sensor ?? config.conteo;
  const pasoColocacion = config.sensor?.paso ?? config.escenaColocada?.paso;
  if (ubicacion) {
    escena.style.setProperty("--target-x", ubicacion.x + "%");
    escena.style.setProperty("--target-y", ubicacion.y + "%");
  }
  if (config.sensor) {
    const sensor = new Image();
    sensor.alt = "";
    sensor.src = config.sensor.archivo;
    sensor.className = "exam-instrument " + config.sensor.clase;
    escena.append(sensor);
  }
  main.querySelector("[data-atencion]")!.addEventListener("click", () => {
    if (confirmando) return;
    window.clearInterval(reloj);
    volver();
  });

  function fase(codigo: string, nombre: string, mensaje: string): void {
    titulo.textContent = config.nombre + " · " + nombre;
    estado.textContent = mensaje;
    escena.dataset.phase = codigo;
  }

  function boton(texto: string, accion: () => void, clase = "button"): HTMLButtonElement {
    const elemento = document.createElement("button");
    elemento.type = "button";
    elemento.className = clase;
    elemento.textContent = texto;
    elemento.addEventListener("click", accion);
    return elemento;
  }

  function inicial(): void {
    fase("initial", "Inicial", "Sin medir");
    escena.dataset.positioned = "false";
    delete escena.dataset.breath;
    escena.querySelector(".chest-motion")?.remove();
    controles.innerHTML = `
      <h2>${config.preparar}</h2>
      <ol class="step-preview">${config.pasos.map((paso) => `<li>${paso}</li>`).join("")}</ol>
      <p class="small-text muted">Duración de la simulación: ${config.duracion} s.</p>
      <div class="button-row"></div>`;
    controles.querySelector(".button-row")!.append(
      boton(config.preparar, preparar, "button primary"), boton("Histórico", mostrarHistorico),
    );
  }

  function preparar(): void {
    fase("prepare", "Preparación", "Preparando");
    escena.dataset.positioned = String(pasoColocacion === "preparacion");
    controles.innerHTML = `
      <div class="stack"><h2>${config.preparar}</h2>
        <fieldset><legend>Confirma la preparación</legend>
        ${config.pasos.map((paso, indice) => `
          <label class="check" for="step-${indice}">
            <input type="checkbox" id="step-${indice}"><span>${paso}</span>
          </label>`).join("")}</fieldset>
        <div class="button-row"></div>
      </div>`;
    const iniciar = boton(config.iniciar, medir, "button primary");
    iniciar.disabled = true;
    controles.querySelector(".button-row")!.append(iniciar, boton("Cancelar preparación", inicial));
    const casillas = Array.from(controles.querySelectorAll("input"));
    for (const casilla of casillas) {
      casilla.addEventListener("change", () => {
        if (typeof pasoColocacion === "number") {
          escena.dataset.positioned = String(casillas[pasoColocacion].checked);
        }
        iniciar.disabled = !casillas.every((paso) => paso.checked);
      });
    }
    casillas[0]?.focus();
  }

  function medir(): void {
    fase("running", "En curso", "Acción en curso");
    escena.dataset.positioned = "true";
    controles.innerHTML = `
      <h2>${config.nombre} en curso</h2><p>${config.instrucciones}</p>
      <div class="stack tight">
        <progress max="${config.duracion}" value="0" aria-label="Progreso del procedimiento"></progress>
        <p class="procedure-progress muted" data-tiempo>${config.duracion} s restantes</p>
      </div>
      <div class="button-row"></div>`;
    const barra = controles.querySelector("progress")!;
    const tiempo = controles.querySelector("[data-tiempo]")!;
    const acciones = controles.querySelector(".button-row")!;
    const cancelar = boton("Cancelar acción", () => {
      window.clearInterval(reloj);
      inicial();
    }, "button danger");
    acciones.append(cancelar);
    ciclos = 0;
    let contar: HTMLButtonElement | undefined;
    let indicacion: HTMLElement | undefined;
    let ultimoMarcado = 0;
    let completados = 0;
    const comienzo = performance.now();
    if (config.conteo) {
      const movimiento = foto.cloneNode(true) as HTMLImageElement;
      movimiento.alt = "";
      movimiento.classList.add("chest-motion");
      escena.append(movimiento);
      const contador = document.createElement("strong");
      contador.id = "breath-count";
      contador.textContent = "0 ciclos marcados";
      contar = boton("Contar ciclo", () => {
        if (completados <= ultimoMarcado) return;
        ultimoMarcado = completados;
        ciclos += 1;
        contador.textContent = ciclos === 1 ? "1 ciclo marcado" : ciclos + " ciclos marcados";
        contar?.setAttribute("aria-disabled", "true");
      });
      contar.setAttribute("aria-disabled", "true");
      const panel = document.createElement("div");
      panel.className = "inset stack tight";
      indicacion = document.createElement("p");
      indicacion.textContent = "Inicia la observación del tórax.";
      panel.append(indicacion, contador, contar);
      acciones.before(panel);
    }
    const clave = crypto.randomUUID();
    reloj = window.setInterval(() => {
      if (!controles.isConnected) {
        window.clearInterval(reloj);
        return;
      }
      const transcurrido = (performance.now() - comienzo) / 1000;
      const segundos = Math.min(config.duracion, transcurrido);
      barra.value = segundos;
      const restantes = Math.ceil(config.duracion - segundos);
      tiempo.textContent = restantes > 0 ? restantes + " s restantes" : "Marca el último ciclo.";
      if (config.conteo && contar && indicacion) {
        const ciclo = segundos * config.conteo.ciclos / config.duracion;
        completados = Math.floor(ciclo);
        const sube = segundos < config.duracion && ciclo % 1 < 0.5;
        escena.dataset.breath = sube ? "up" : "down";
        const habilitado = completados > ultimoMarcado;
        indicacion.textContent = habilitado ? "Ciclo completo. Puedes marcarlo." : sube ? "El tórax se eleva." : "El tórax desciende.";
        contar.setAttribute("aria-disabled", String(!habilitado));
      }
      const termino = config.duracion + (config.conteo ? 1 : 0);
      if (transcurrido >= termino) {
        window.clearInterval(reloj);
        cancelar.disabled = true;
        if (contar) contar.disabled = true;
        void confirmar(clave);
      }
    }, 200);
  }

  async function confirmar(clave: string): Promise<void> {
    if (confirmando) return;
    confirmando = true;
    estado.textContent = "Guardando la lectura";
    const cuerpo = { instrumento: config.id, accion: "toma_completa", ...(config.conteo ? { ciclos } : {}) };
    try {
      mostrarResultado(await api<Medicion>(ruta, escribir("POST", cuerpo, clave)));
    } catch (error) {
      mostrarError(error, () => { void confirmar(clave); });
    } finally {
      confirmando = false;
    }
  }

  function mostrarResultado(medicion: Medicion): void {
    fase("result", "Resultado", "Resultado listo");
    escena.querySelector(".chest-motion")?.remove();
    delete escena.dataset.breath;
    controles.innerHTML = `
      <h2>Lectura de ${config.nombre.toLowerCase()}</h2><p data-valor></p>
      <p class="small-text muted" data-origen></p><p role="status" data-registro></p>
      <div class="button-row"></div>`;
    controles.querySelector("[data-valor]")!.textContent = textoMedicion(medicion);
    controles.querySelector("[data-origen]")!.textContent = [medicion.origen, medicion.momento].filter(Boolean).join(" · ");
    const registro = controles.querySelector("[data-registro]")!;
    const clave = crypto.randomUUID();
    const registrar = boton("Registrar en ficha", async () => {
      registrar.disabled = true;
      try {
        const dato = await api<Medicion>(
          ruta + "/" + encodeURIComponent(medicion.medicion_id) + "/ficha", escribir("PUT", {}, clave));
        registro.textContent = dato.en_ficha ? "Registrada en ficha" : "";
      } catch (error) {
        registro.textContent = error instanceof Error ? error.message : "No se pudo guardar la lectura.";
      } finally {
        registrar.disabled = false;
      }
    }, "button primary");
    registro.textContent = medicion.en_ficha ? "Registrada en ficha" : "";
    controles.querySelector(".button-row")!.append(registrar, boton("Histórico", mostrarHistorico), boton("Preparar otra toma", inicial));
  }

  async function mostrarHistorico(): Promise<void> {
    fase("history", "Histórico", "Histórico de tomas");
    controles.innerHTML = `<h2>Histórico de ${config.nombre.toLowerCase()}</h2>`;
    try {
      const lecturas = await api<Medicion[]>(ruta + "?instrumento=" + encodeURIComponent(config.id));
      const lista = document.createElement("ul");
      for (const medicion of lecturas) {
        const fila = document.createElement("li");
        fila.dataset.medicionId = medicion.medicion_id;
        fila.textContent = textoMedicion(medicion);
        lista.append(fila);
      }
      controles.append(lista, boton("Preparar otra toma", inicial));
    } catch (error) {
      mostrarError(error, mostrarHistorico);
    }
  }

  function mostrarError(error: unknown, reintentar: () => void): void {
    const mensaje = document.createElement("p");
    mensaje.setAttribute("role", "status");
    mensaje.textContent = error instanceof Error ? error.message : "No se recibió la lectura.";
    controles.replaceChildren(mensaje, boton("Reintentar", reintentar));
  }

  inicial();
  titulo.focus();
}
