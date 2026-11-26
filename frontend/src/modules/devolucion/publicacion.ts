import { api, escribir } from "../../app/api.js";
import type { Revision } from "./revision.js";

export interface Publicacion {
  evaluacion_id: string;
  numero: number;
  revision: number;
  publicada_en: string;
}

type Devolucion = { estado: "no_disponible" | "post_pendiente" | "pendiente" } | {
  estado: "publicada";
  revision: number;
  publicada_en: string;
  observaciones: Revision["resultado"]["observaciones"];
  prioridad: Revision["resultado"]["prioridad"];
  puntaje: Revision["puntaje"];
};

const NOMBRES: Record<string, string> = {
  entrevista: "Entrevista",
  valoracion: "Valoración",
  decisiones: "Decisiones",
  evidencias: "Uso de evidencias",
  hipotesis: "Hipótesis",
};

export const tituloCriterio = (id: string): string => NOMBRES[id] ?? id.replace(/^hipotesis-(\d+)$/, "Hipótesis $1");

export function montarPublicacion(
  controles: Element, ruta: string, revision: Revision, previa: Publicacion | null,
): void {
  const panel = document.createElement("div");
  panel.className = "stack tight";
  const boton = document.createElement("button");
  boton.className = "button";
  boton.type = "button";
  boton.textContent = previa ? "Publicar rectificación" : "Publicar devolución";
  const estado = document.createElement("p");
  estado.setAttribute("role", "status");
  if (previa) estado.textContent = `Revisión publicada ${previa.revision}.`;
  const pendiente = revision.resultado.observaciones.some((o) => o.juicio === "revision_docente" || o.incidencia);
  boton.disabled = pendiente || previa?.revision === revision.revision;
  if (pendiente) {
    const aviso = document.createElement("p");
    aviso.textContent = "Resuelve los criterios pendientes de revisión antes de publicar.";
    panel.append(aviso);
  }
  panel.append(boton, estado);
  controles.append(panel);
  const clave = crypto.randomUUID();
  boton.addEventListener("click", async () => {
    boton.disabled = true;
    try {
      const publicada = await api<Publicacion>(ruta + "/publicacion", escribir("POST", { revision: revision.revision }, clave));
      estado.textContent = `Revisión publicada ${publicada.revision}.`;
    } catch (error) {
      estado.textContent = error instanceof Error ? error.message : "No se pudo publicar la devolución.";
      boton.disabled = false;
    }
  });
}

export async function mostrarDevolucion(panel: HTMLElement, intento: string): Promise<void> {
  try {
    const datos = await api<Devolucion>(`/intentos/${encodeURIComponent(intento)}/devolucion`);
    if (!panel.isConnected) return;
    if (datos.estado !== "publicada") {
      panel.textContent = datos.estado === "post_pendiente" ? "Resuelve el post para consultar la devolución."
        : "Devolución docente pendiente.";
      return;
    }
    const titulo = document.createElement("h3");
    titulo.textContent = "Tu devolución";
    const resumen = document.createElement("p");
    const puntos = datos.puntaje.total_centesimas;
    resumen.textContent = `Revisión publicada ${datos.revision}.` + (puntos === null ? ""
      : ` Puntaje ${new Intl.NumberFormat("es-CL").format(puntos / 100)} de 100.`);
    panel.append(titulo, resumen);
    if (datos.prioridad) agregarNota(panel, "Comentario prioritario", `${datos.prioridad.accion} ${datos.prioridad.motivo}`);
    for (const criterio of datos.observaciones) {
      agregarNota(panel, tituloCriterio(criterio.criterio_id), criterio.explicacion + " " + criterio.mejora);
    }
  } catch (error) {
    panel.textContent = error instanceof Error ? error.message : "No se pudo consultar la devolución.";
  }
}

function agregarNota(panel: HTMLElement, nombre: string, texto: string): void {
  const nota = document.createElement("article");
  nota.className = "tile";
  const titulo = document.createElement("h4");
  titulo.textContent = nombre;
  const contenido = document.createElement("p");
  contenido.className = "preserve-lines";
  contenido.textContent = texto;
  nota.append(titulo, contenido);
  panel.append(nota);
}
