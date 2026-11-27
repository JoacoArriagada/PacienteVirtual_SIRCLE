import { api, escribir } from "../../app/api.js";
import { montarSugerencias, type Sugerencia } from "./sugerencias.js";

export interface Turno {
  turno_id: string;
  intento_id: string;
  operacion_id: string;
  orden: number;
  hablante: string;
  pregunta: string;
  respuesta: string;
  tipo_respuesta: string;
}

export interface VozEntrevista {
  hablante: string;
  sugerencias: Sugerencia[];
}

export function montarChat(
  contenedor: HTMLElement, intento: string, guardados: Turno[], sugerencias: Sugerencia[],
  inicio: { paciente: string; motivo: string }, alResponder: () => void,
  voces?: VozEntrevista[],
): (id: string) => void {
  contenedor.innerHTML = `
    <h2 class="visually-hidden">Entrevista</h2>
    <div id="chat-log" class="chat-log" tabindex="0" role="log" aria-live="polite"></div>
    <div data-formulario></div>`;
  const lista = contenedor.querySelector<HTMLElement>("#chat-log")!;
  lista.setAttribute("aria-label", `Conversación con ${inicio.paciente}`);
  const mostrados = new Map<string, HTMLElement>();
  const motivo = inicio.motivo === "Falta de aire" ? "Me falta el aire." : inicio.motivo;
  if (motivo) lista.append(burbuja("patient", inicio.paciente, motivo));

  function agregarTurno(turno: Turno): void {
    if (mostrados.has(turno.turno_id)) return;
    const intercambio = document.createElement("div");
    intercambio.dataset.turno = turno.turno_id;
    intercambio.dataset.tipoRespuesta = turno.tipo_respuesta;
    if (turno.pregunta) intercambio.append(burbuja("student", "Estudiante", turno.pregunta));
    intercambio.append(burbuja(turno.hablante === "Sistema" ? "system" : "patient", turno.hablante, turno.respuesta));
    lista.append(intercambio);
    mostrados.set(turno.turno_id, intercambio);
  }

  guardados.forEach(agregarTurno);
  const formulario = contenedor.querySelector<HTMLElement>("[data-formulario]")!;
  const selector = voces && voces.length > 1 ? document.createElement("select") : undefined;
  if (selector && voces) {
    const etiqueta = document.createElement("label");
    etiqueta.className = "field";
    etiqueta.textContent = "Hablar con";
    for (const voz of voces) selector.add(new Option(voz.hablante, voz.hablante));
    etiqueta.append(selector);
    formulario.before(etiqueta);
    selector.addEventListener("change", () => prepararFormulario(voces.find((voz) => voz.hablante === selector.value)));
  }
  prepararFormulario(voces?.[0]);

  function prepararFormulario(voz?: VozEntrevista): void {
    montarSugerencias(formulario, voz?.sugerencias ?? sugerencias, async (pregunta, clave) => {
      const ruta = "/intentos/" + encodeURIComponent(intento) + "/turnos";
      const cuerpo = voz ? { ...pregunta, destinatario: voz.hablante } : pregunta;
      if (selector) selector.disabled = true;
      try {
        const turno = await api<Turno>(ruta, escribir("POST", cuerpo, clave));
        agregarTurno(turno);
        lista.scrollTop = lista.scrollHeight;
        alResponder();
      } finally {
        if (selector) selector.disabled = false;
      }
    });
  }

  return (id) => {
    const mensaje = mostrados.get(id)?.lastElementChild as HTMLElement | undefined;
    if (!mensaje) return;
    lista.querySelector(".highlighted")?.classList.remove("highlighted");
    mensaje.classList.add("highlighted");
    mensaje.tabIndex = -1;
    mensaje.scrollIntoView({ block: "nearest" });
    mensaje.focus({ preventScroll: true });
  };
}

function burbuja(rol: string, hablante: string, texto: string): HTMLElement {
  const bloque = document.createElement("article");
  bloque.className = "bubble " + rol;
  const titulo = document.createElement("header");
  const nombre = document.createElement("strong");
  nombre.textContent = hablante;
  titulo.append(nombre);
  const mensaje = document.createElement("p");
  mensaje.textContent = texto;
  bloque.append(titulo, mensaje);
  return bloque;
}
