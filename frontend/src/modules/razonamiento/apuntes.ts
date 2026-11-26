import { api, ErrorHttp, escribir } from "../../app/api.js";
import { textoMedicion } from "../valoracion/instrumento.js";

export interface Apunte {
  apunte_id: string;
  autor_id: string;
  intencion: string;
  momento: string;
  revision: number;
}

export interface Evidencia {
  tipo: string;
  origen_id: string;
  categoria: string;
  autor: string;
  origen: string;
  texto?: string;
  instrumento?: string;
  valor?: Record<string, number>;
  unidad?: string;
  momento?: string;
}

interface Registros {
  revision: number;
  apuntes?: Apunte[];
  evidencias?: Evidencia[];
}

const categorias: Record<string, string> = {
  declaracion: "Declaración",
  medicion: "Dato medido",
  observacion: "Observación",
  orientacion: "Orientación",
  informe: "Informe",
  interpretacion: "Interpretación",
};

export function mostrarDato(evidencia: Evidencia): string {
  const { texto, instrumento, valor, unidad } = evidencia;
  if (instrumento && valor && unidad) return textoMedicion({ instrumento, valor, unidad });
  return texto ?? "";
}

export function montarApuntes(panel: HTMLElement, intento: string, registros: Registros): void {
  const ruta = "/intentos/" + encodeURIComponent(intento);
  panel.className = "panel stack";
  panel.setAttribute("aria-labelledby", "titulo-apuntes");
  panel.innerHTML = `
    <h2 id="titulo-apuntes">Apuntes y evidencias</h2>
    <form class="stack tight">
      <label for="apunte">Apunte de la atención</label>
      <textarea id="apunte" rows="3" maxlength="1500" required></textarea>
      <div><button class="button primary" type="submit">Guardar apunte</button></div>
    </form>
    <p role="status" data-estado hidden></p>
    <div class="stack tight" data-apuntes></div>
    <details class="disclosure">
      <summary>Evidencias del intento</summary>
      <p role="status" data-carga hidden></p>
      <ul class="stack tight" data-evidencias></ul>
    </details>`;
  const formulario = panel.querySelector("form")!;
  const entrada = panel.querySelector("textarea")!;
  const guardar = formulario.querySelector("button")!;
  const estado = panel.querySelector<HTMLElement>("[data-estado]")!;
  const lista = panel.querySelector<HTMLElement>("[data-apuntes]")!;
  const evidencias = panel.querySelector<HTMLElement>("[data-evidencias]")!;
  const carga = panel.querySelector<HTMLElement>("[data-carga]")!;
  const detalle = panel.querySelector("details")!;
  let apuntes: Apunte[] = registros.apuntes ?? [];
  let envio: { apunte: string; revision_esperada: number; clave: string } | undefined;
  let guardando = false;

  function pintarApuntes(): void {
    lista.replaceChildren();
    for (const apunte of apuntes) {
      const fila = document.createElement("article");
      fila.className = "tile stack tight";
      fila.dataset.apunteId = apunte.apunte_id;
      const texto = document.createElement("p");
      texto.className = "preserve-lines";
      texto.textContent = apunte.intencion;
      const autor = document.createElement("p");
      autor.className = "small-text muted";
      autor.textContent = [apunte.autor_id, apunte.momento].join(" · ");
      fila.append(texto, autor);
      lista.append(fila);
    }
  }

  function pintarEvidencias(elementos: Evidencia[]): void {
    evidencias.replaceChildren();
    for (const evidencia of elementos) {
      const fila = document.createElement("li");
      fila.dataset.origenId = evidencia.origen_id;
      const categoria = document.createElement("strong");
      categoria.textContent = categorias[evidencia.categoria] ?? evidencia.categoria;
      const texto = document.createElement("p");
      texto.textContent = mostrarDato(evidencia);
      const origen = document.createElement("p");
      origen.className = "small-text muted";
      origen.textContent = [evidencia.origen, evidencia.autor, evidencia.origen_id].join(" · ");
      fila.append(categoria, texto, origen);
      evidencias.append(fila);
    }
  }

  async function recargarEvidencias(): Promise<void> {
    try {
      const actual = await api<Registros>(ruta);
      pintarEvidencias(actual.evidencias ?? []);
      carga.hidden = true;
    } catch (error) {
      carga.textContent = error instanceof Error ? error.message : "No se pudieron actualizar las evidencias.";
      carga.hidden = false;
    }
  }

  detalle.addEventListener("toggle", () => {
    if (detalle.open) void recargarEvidencias();
  });
  formulario.addEventListener("submit", async (evento) => {
    evento.preventDefault();
    const apunte = entrada.value.trim();
    if (!apunte || guardando) return;
    guardando = guardar.disabled = entrada.readOnly = true;
    try {
      if (envio?.apunte !== apunte) {
        const actual = await api<Registros>(ruta);
        envio = { apunte, revision_esperada: actual.revision, clave: crypto.randomUUID() };
        apuntes = actual.apuntes ?? [];
      }
      const { clave, ...datos } = envio;
      const confirmado = await api<Apunte>(ruta + "/razonamiento", escribir("PUT", datos, clave));
      apuntes = [...apuntes.filter((item) => item.apunte_id !== confirmado.apunte_id), confirmado];
      pintarApuntes();
      entrada.value = "";
      envio = undefined;
      estado.textContent = "Apunte guardado.";
      if (detalle.open) void recargarEvidencias();
    } catch (error) {
      if (error instanceof ErrorHttp && error.estado === 409) envio = undefined;
      estado.textContent = error instanceof Error ? error.message : "No se pudo guardar el apunte.";
    } finally {
      guardando = guardar.disabled = entrada.readOnly = false;
      estado.hidden = false;
      entrada.focus();
    }
  });
  pintarApuntes();
  pintarEvidencias(registros.evidencias ?? []);
}
