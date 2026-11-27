import { api, ErrorHttp, escribir } from "../../app/api.js";
import { iconoValoracion } from "../valoracion/brazo.js";
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
}

export function mostrarDato(evidencia: Evidencia): string {
  const { texto, instrumento, valor, unidad } = evidencia;
  if (instrumento && valor && unidad) return textoMedicion({ instrumento, valor, unidad });
  return texto ?? "";
}

export function montarApuntes(
  panel: HTMLElement, intento: string, registros: Registros, consultarEvidencia: () => void, alGuardar: () => void,
): void {
  const ruta = "/intentos/" + encodeURIComponent(intento);
  panel.innerHTML = `
    <h2>Mis apuntes</h2>
    <div class="empty" data-sin-apuntes><strong>Sin apuntes</strong>Añade tus observaciones en puntos breves.</div>
    <div class="stack tight notebook-scroll" data-apuntes hidden></div>
    <button class="text-button notebook-link" type="button" data-consultar-evidencia>
      ${iconoValoracion("ficha-clinica", 20)}<span>Consultar evidencia</span>
    </button>
    <form class="note-form stack tight">
      <label class="small-text" for="apunte">Añadir un punto</label>
      <textarea id="apunte" rows="3" maxlength="1500" placeholder="Escribe una nota breve…" required></textarea>
      <div class="button-row end"><button class="button primary small" type="submit"><span aria-hidden="true">＋</span> Añadir punto</button></div>
      <p class="small-text" role="status" data-estado hidden></p>
    </form>`;
  const formulario = panel.querySelector("form")!;
  const entrada = panel.querySelector("textarea")!;
  const guardar = formulario.querySelector("button")!;
  const estado = panel.querySelector<HTMLElement>("[data-estado]")!;
  const lista = panel.querySelector<HTMLElement>("[data-apuntes]")!;
  const vacio = panel.querySelector<HTMLElement>("[data-sin-apuntes]")!;
  let apuntes: Apunte[] = registros.apuntes ?? [];
  let envio: { apunte: string; revision_esperada: number; clave: string } | undefined;
  let guardando = false;

  function pintarApuntes(): void {
    vacio.hidden = apuntes.length > 0;
    lista.hidden = apuntes.length === 0;
    lista.replaceChildren();
    for (const apunte of apuntes) {
      const fila = document.createElement("article");
      fila.className = "tile stack tight";
      fila.dataset.apunteId = apunte.apunte_id;
      const texto = document.createElement("p");
      texto.className = "preserve-lines";
      texto.textContent = apunte.intencion;
      const momento = document.createElement("time");
      momento.className = "source-label";
      momento.dateTime = apunte.momento;
      momento.textContent = new Date(apunte.momento).toLocaleString("es-CL");
      fila.append(texto, momento);
      lista.append(fila);
    }
  }

  panel.querySelector("[data-consultar-evidencia]")!.addEventListener("click", consultarEvidencia);
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
      alGuardar();
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
}
