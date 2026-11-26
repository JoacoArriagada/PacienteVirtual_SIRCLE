import { api } from "../../app/api.js";

type Distribucion = Record<string, number>;

interface Informe {
  invitados: number;
  sin_intento: number;
  pre_respondidos: number;
  post_respondidos: number;
  pares_completos: number;
  intentos: {
    estudiante_id: string;
    intento_id: string;
    version: string;
    secuencia: string;
    repeticion: boolean;
    pre: string;
    post: string;
  }[];
  grupos: {
    version: string;
    repeticion: boolean;
    percepcion: {
      enunciado: string; pre: Distribucion; post: Distribucion; cambio: Distribucion;
      faltantes_pre: number; faltantes_post: number;
    }[];
    razonamiento: {
      secuencia: string; momento: string; forma: string; porcentaje: Distribucion; faltantes_por_item: number[];
    }[];
    cambio_razonamiento: Distribucion;
    utilidad: { enunciado: string; post: Distribucion; faltantes_post: number }[];
  }[];
}

export async function mostrarInforme(panel: HTMLElement, actividad: string): Promise<void> {
  panel.textContent = "Cargando resultados educativos…";
  try {
    const informe = await api<Informe>(`/actividades/${encodeURIComponent(actividad)}/resultados-educativos`);
    if (!panel.isConnected) return;
    panel.replaceChildren();
    texto(panel, "p", `${informe.invitados} invitados · ${informe.sin_intento} sin intento · ${informe.pre_respondidos} pre respondidos · ${informe.post_respondidos} post respondidos · ${informe.pares_completos} pares completos`);
    if (!informe.intentos.length) return;
    tabla(panel, ["Identidad e intento", "Versión y orden", "Pre", "Post"], informe.intentos.map((intento) => [
      `${intento.estudiante_id} · ${intento.intento_id}`,
      `${intento.version} · ${intento.secuencia} · ${intento.repeticion ? "Repetición" : "Primera exposición"}`,
      intento.pre, intento.post,
    ]));
    for (const grupo of informe.grupos) {
      texto(panel, "h3", grupo.version + (grupo.repeticion ? " · Repeticiones" : " · Primera exposición"));
      texto(panel, "h4", "Percepción por ítem (1–5)");
      tabla(panel, ["Enunciado", "Pre", "Post", "Cambio pareado"], grupo.percepcion.map((item) => [
        item.enunciado, conFaltantes(item.pre, item.faltantes_pre), conFaltantes(item.post, item.faltantes_post),
        resumir(item.cambio),
      ]));
      texto(panel, "h4", "Razonamiento por forma y orden");
      tabla(panel, ["Secuencia", "Momento", "Forma", "Acierto (sobre dos)", "Faltantes por ítem"], grupo.razonamiento.map((fila) => [
        fila.secuencia, fila.momento, fila.forma, resumir(fila.porcentaje, "%"), fila.faltantes_por_item.join(" / "),
      ]));
      texto(panel, "p", "Cambio pareado en puntos porcentuales · " + resumir(grupo.cambio_razonamiento));
      texto(panel, "h4", "Utilidad percibida en el post (1–5)");
      tabla(panel, ["Enunciado", "Post"], grupo.utilidad.map((item) => [
        item.enunciado, conFaltantes(item.post, item.faltantes_post),
      ]));
    }
  } catch (error) {
    panel.textContent = error instanceof Error ? error.message : "No se pudo consultar el informe educativo.";
  }
}

function resumir(datos: Distribucion, unidad = ""): string {
  return Object.entries(datos).sort(([a], [b]) => Number(a) - Number(b))
    .map(([valor, cantidad]) => `${valor}${unidad} (${cantidad})`).join(" · ") || "Sin respuestas";
}

function conFaltantes(datos: Distribucion, faltantes: number): string {
  return `${resumir(datos)} · ${faltantes} faltantes`;
}

function texto(padre: HTMLElement, etiqueta: string, contenido: string): void {
  const elemento = document.createElement(etiqueta);
  elemento.textContent = contenido;
  padre.append(elemento);
}

function tabla(padre: HTMLElement, columnas: string[], filas: string[][]): void {
  const contenedor = document.createElement("div");
  contenedor.className = "table-wrap";
  const tabla = document.createElement("table");
  const cabecera = tabla.createTHead().insertRow();
  for (const nombre of columnas) texto(cabecera, "th", nombre);
  const cuerpo = tabla.createTBody();
  for (const fila of filas) {
    const tr = cuerpo.insertRow();
    for (const dato of fila) tr.insertCell().textContent = dato;
  }
  contenedor.append(tabla);
  padre.append(contenedor);
}
