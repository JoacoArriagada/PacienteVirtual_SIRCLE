import { api, escribir } from "../../app/api.js";
import { iconoValoracion, retratoLR } from "./brazo.js";
import { AudioAuscultacion } from "./audio-auscultacion.js";

export interface Zona {
  id: string;
  etiqueta: string;
  imagen: string;
  x: number;
  y: number;
  modo?: string;
}

export interface InstrumentoZonado {
  id: string;
  nombre: string;
  metodo: string;
  icono: string;
  preparar: string;
  iniciar: string;
  duracion: number;
  instrucciones: string;
  pasos: readonly string[];
  zonas: readonly Zona[];
  accesorio?: { archivo: string; clase: string };
  modos?: readonly { id: string; etiqueta: string }[];
  audio?: boolean;
}

interface Exploracion {
  exploracion_id: string;
  etiqueta: string;
  estado: string;
  momento: string;
  hallazgo?: { hallazgo_id: string; texto: string; origen: string };
}

export function mostrarExploracion(main: HTMLElement, intento: string, equipo: InstrumentoZonado, volver: () => void): void {
  main.dataset.view = "procedure";
  main.innerHTML = `
    <section class="view procedure-view" aria-labelledby="page-title">
      <div class="page-heading with-back">
        <div class="heading-back"><button class="button" type="button" data-volver>
          ${iconoValoracion("vista-volver", 20)}<span>Volver a la atención</span>
        </button></div>
        <div class="heading-copy"><h1 id="page-title" tabindex="-1"></h1><p class="muted">${equipo.metodo} · Simulación</p></div>
      </div>
      <div class="procedure-grid">
        <div class="panel">
          <div class="procedure-portrait"><img src="${retratoLR}" class="portrait" width="38" height="38" alt="">
            <strong>LR</strong><span class="muted">· +00 min</span>
          </div>
          <figure class="stack tight">
            <div id="device-art" class="device-art" data-phase="initial" data-positioned="false">
              <img class="procedure-photo" alt="Escena clínica de LR" width="756" height="504">
            </div>
            <figcaption class="small-text muted" data-zona></figcaption>
          </figure>
        </div>
        <section class="panel stack" aria-label="Controles del instrumento">
          <div class="procedure-state"><span>Estado del procedimiento</span><strong data-estado></strong></div>
          <div data-controles></div>
        </section>
      </div>
    </section>`;
  const escena = main.querySelector<HTMLElement>("#device-art")!;
  const titulo = main.querySelector<HTMLElement>("#page-title")!;
  const estado = main.querySelector<HTMLElement>("[data-estado]")!;
  const etiqueta = main.querySelector<HTMLElement>("[data-zona]")!;
  const controles = main.querySelector<HTMLElement>("[data-controles]")!;
  const foto = escena.querySelector("img")!;
  let zona: Zona | undefined;
  let modo = equipo.modos?.[0].id;
  const audio = equipo.audio ? new AudioAuscultacion() : undefined;
  let reloj: number | undefined;
  let guardando = false;
  main.querySelector("[data-volver]")!.addEventListener("click", () => {
    if (guardando) return;
    window.clearInterval(reloj);
    audio?.detener();
    volver();
  });

  function boton(texto: string, accion: () => void, clase = "button", icono = ""): HTMLButtonElement {
    const boton = document.createElement("button");
    boton.type = "button";
    boton.className = clase;
    const etiqueta = document.createElement("span");
    etiqueta.textContent = texto;
    boton.innerHTML = icono;
    boton.append(etiqueta);
    boton.addEventListener("click", accion);
    return boton;
  }

  function fase(codigo: string, nombre: string, mensaje: string): void {
    titulo.textContent = equipo.nombre + " · " + nombre;
    estado.textContent = mensaje;
    escena.dataset.phase = codigo;
    escena.dataset.positioned = String(codigo === "running" || codigo === "result");
    escena.querySelector(".exam-target")?.remove();
    escena.querySelector(".exam-instrument")?.remove();
    const vista = zona ?? equipo.zonas.find((punto) => punto.modo === modo) ?? equipo.zonas[0];
    foto.src = vista.imagen;
    escena.style.setProperty("--target-x", vista.x + "%");
    escena.style.setProperty("--target-y", vista.y + "%");
    etiqueta.textContent = (zona?.etiqueta ?? equipo.metodo) + " · Escena de simulación";
    if (zona && (codigo === "prepare" || equipo.id === "inspeccion")) {
      const objetivo = document.createElement("span");
      objetivo.className = "exam-target";
      objetivo.setAttribute("aria-hidden", "true");
      escena.append(objetivo);
    }
    if (zona && equipo.accesorio) {
      const accesorio = new Image();
      accesorio.className = "exam-instrument " + equipo.accesorio.clase;
      accesorio.src = equipo.accesorio.archivo;
      accesorio.alt = "";
      escena.append(accesorio);
    }
  }

  function inicial(): void {
    zona = undefined;
    modo = equipo.modos?.[0].id;
    fase("initial", "Inicial", "Sin explorar");
    controles.innerHTML = `
      <h2>${equipo.preparar}</h2>
      <ol class="step-preview">${equipo.pasos.map((paso) => `<li>${paso}</li>`).join("")}</ol>
      <p class="small-text muted">Duración de la simulación: ${equipo.duracion} s.</p>`;
    controles.append(boton(equipo.preparar, preparar, "button primary"));
  }

  function preparar(): void {
    fase("prepare", "Preparación", "Preparando");
    controles.innerHTML = `
      <div class="stack"><h2>${equipo.preparar}</h2>
        <fieldset><legend>Confirma la preparación</legend>
        ${equipo.pasos.map((paso, indice) => `<label class="check" for="step-${indice}">
          <input id="step-${indice}" type="checkbox"><span>${paso}</span></label>`).join("")}</fieldset>
        <div class="stack tight"><h3>Selecciona la zona</h3><div class="zone-list"></div></div>
        <div class="button-row"></div>
      </div>`;
    const comenzar = boton(equipo.iniciar, medir, "button primary",
      '<svg class="icon" viewBox="0 0 24 24" aria-hidden="true" focusable="false"><path d="m8 5 12 7-12 7z"></path></svg>');
    comenzar.disabled = true;
    controles.querySelector(".button-row")!.append(comenzar,
      boton("Cancelar preparación", inicial, "button", iconoValoracion("cancelar-d939d1e1f0", 20)));
    const lista = controles.querySelector(".zone-list")!;
    const pasos = Array.from(controles.querySelectorAll("input"));
    const zonas = new Map<Zona, HTMLButtonElement>();
    const modos = new Map<string, HTMLButtonElement>();

    function actualizar(): void {
      comenzar.disabled = !zona || !pasos.every((paso) => paso.checked);
      for (const [opcion, control] of zonas) control.setAttribute("aria-pressed", String(opcion === zona));
      for (const [opcion, control] of modos) control.setAttribute("aria-pressed", String(opcion === modo));
    }
    function pintarZonas(): void {
      zonas.clear();
      lista.replaceChildren();
      for (const opcion of equipo.zonas.filter((punto) => punto.modo === modo)) {
        const control = boton(opcion.etiqueta, () => {
          zona = opcion;
          fase("prepare", "Preparación", "Preparando");
          actualizar();
        }, "button small", '<svg class="icon" viewBox="0 0 24 24" aria-hidden="true" focusable="false"><circle cx="12" cy="12" r="8"></circle><circle cx="12" cy="12" r="3"></circle><path d="M12 2v3m0 14v3M2 12h3m14 0h3"></path></svg>');
        control.id = "zone-" + opcion.id;
        zonas.set(opcion, control);
        lista.append(control);
      }
    }
    if (equipo.modos) {
      const selector = document.createElement("div");
      selector.className = "tabs";
      selector.setAttribute("role", "group");
      selector.setAttribute("aria-label", "Vista de auscultación");
      for (const vista of equipo.modos) {
        const control = boton(vista.etiqueta, () => {
          modo = vista.id;
          zona = undefined;
          fase("prepare", "Preparación", "Preparando");
          pintarZonas();
          actualizar();
        }, "tab");
        modos.set(vista.id, control);
        selector.append(control);
      }
      controles.querySelector("fieldset")!.after(selector);
    }
    pintarZonas();
    for (const paso of pasos) paso.addEventListener("change", actualizar);
    actualizar();
    pasos[0]?.focus();
  }

  function medir(): void {
    const elegida = zona;
    if (!elegida) return;
    fase("running", "En curso", "Acción en curso");
    controles.innerHTML = `
      <h2>${equipo.nombre} en curso</h2><p>${equipo.instrucciones}</p>
      <progress max="${equipo.duracion}" value="0" aria-label="Progreso del procedimiento"></progress>`;
    const progreso = controles.querySelector("progress")!;
    if (audio) {
      const reproducir = boton("Escuchar audio de práctica", async () => {
        reproducir.disabled = true;
        try {
          const sonando = await audio.alternar(elegida.modo === "cardiaco");
          reproducir.querySelector("span")!.textContent = sonando ? "Detener audio" : "Escuchar audio de práctica";
        } catch {
          estado.textContent = "No se pudo reproducir el audio de práctica.";
        } finally {
          reproducir.disabled = false;
        }
      });
      controles.append(reproducir);
    }
    const cancelar = boton("Cancelar acción", () => {
      window.clearInterval(reloj);
      audio?.detener();
      inicial();
    }, "button danger");
    controles.append(cancelar);
    const comienzo = performance.now();
    const clave = crypto.randomUUID();
    reloj = window.setInterval(() => {
      if (!controles.isConnected) {
        window.clearInterval(reloj);
        audio?.detener();
        return;
      }
      progreso.value = (performance.now() - comienzo) / 1000;
      if (progreso.value >= equipo.duracion) {
        window.clearInterval(reloj);
        audio?.detener();
        cancelar.disabled = true;
        void guardar(elegida, clave);
      }
    }, 200);
  }

  async function guardar(zona: Zona, clave: string): Promise<void> {
    if (guardando) return;
    guardando = true;
    try {
      const dato = await api<Exploracion>(
        "/intentos/" + encodeURIComponent(intento) + "/exploraciones",
        escribir("POST", { instrumento: equipo.id, zona: zona.id, accion: "exploracion_completa" }, clave));
      fase("result", "Resultado", dato.hallazgo ? "Hallazgo disponible" : "Hallazgo pendiente");
      controles.innerHTML = '<h2>Exploración registrada</h2><p data-resultado></p><p class="small-text muted" data-origen></p>';
      controles.querySelector<HTMLElement>("[data-resultado]")!.textContent = dato.hallazgo?.texto
        ?? "La zona se exploró. El hallazgo de esta versión está pendiente de revisión.";
      controles.querySelector<HTMLElement>("[data-origen]")!.textContent =
        [dato.etiqueta, dato.hallazgo?.origen, dato.momento].filter(Boolean).join(" · ");
      controles.append(boton("Explorar otra zona", inicial));
    } catch (error) {
      const aviso = document.createElement("p");
      aviso.setAttribute("role", "status");
      aviso.textContent = error instanceof Error ? error.message : "No se guardó la exploración.";
      controles.replaceChildren(aviso, boton("Reintentar", () => { void guardar(zona, clave); }));
    } finally {
      guardando = false;
    }
  }

  inicial();
  titulo.focus();
}
