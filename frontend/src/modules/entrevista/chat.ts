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

export function montarChat(
  contenedor: HTMLElement, intento: string, guardados: Turno[], sugerencias: Sugerencia[],
  inicio: { paciente: string; motivo: string }, alResponder: () => void,
): (id: string) => void {
  contenedor.innerHTML = `
    <h2 class="visually-hidden">Entrevista</h2>
    <div id="chat-log" class="chat-log" tabindex="0" role="log" aria-live="polite" aria-label="Conversación con LR"></div>
    <div data-formulario></div>`;
  const lista = contenedor.querySelector<HTMLElement>("#chat-log")!;
  const mostrados = new Map<string, HTMLElement>();
  const motivo = inicio.motivo === "Falta de aire" ? "Me falta el aire." : inicio.motivo;
  lista.append(burbuja("patient", inicio.paciente, motivo));

  function agregarTurno(turno: Turno): void {
    if (mostrados.has(turno.turno_id)) return;
    const intercambio = document.createElement("div");
    intercambio.dataset.turno = turno.turno_id;
    intercambio.dataset.tipoRespuesta = turno.tipo_respuesta;
    intercambio.append(
      burbuja("student", "Estudiante", turno.pregunta),
      burbuja(turno.tipo_respuesta === "dato_ausente" ? "system" : "patient", turno.hablante, turno.respuesta),
    );
    lista.append(intercambio);
    mostrados.set(turno.turno_id, intercambio);
  }

  guardados.forEach(agregarTurno);
  montarSugerencias(contenedor.querySelector("[data-formulario]")!, sugerencias, async (pregunta, clave) => {
    const ruta = "/intentos/" + encodeURIComponent(intento) + "/turnos";
    const turno = await api<Turno>(ruta, escribir("POST", pregunta, clave));
    agregarTurno(turno);
    lista.scrollTop = lista.scrollHeight;
    alResponder();
  });

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
