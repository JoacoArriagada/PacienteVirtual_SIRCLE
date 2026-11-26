import { api, escribir } from "../../app/api.js";
import { iconoValoracion } from "../valoracion/brazo.js";
import { JUICIOS, montarPublicacion, tituloCriterio, tituloJuicio, type Publicacion } from "./publicacion.js";
import { mostrarInforme } from "../encuestas/informe.js";

interface Actividad { actividad_id: string; nombre: string }
interface Entrega { estudiante_id: string; intento_id: string; estado: string; evaluacion_id: string | null }
interface Evidencia { id: string; tipo: string; resumen: string }
interface Observacion {
  criterio_id: string;
  actuacion: string;
  evidencias: string[];
  expectativa: string;
  juicio: string;
  explicacion: string;
  mejora: string;
  incidencia?: { motivo: string };
}
interface Credito { criterio_id: string; peso: number; credito: number | null; puntos_ponderados: number | null }

export interface Revision {
  revision: number;
  anterior: number | null;
  autor_id: string | null;
  motivo: string;
  resultado: { observaciones: Observacion[]; prioridad?: { motivo: string; accion: string } };
  puntaje: { peso_aplicable: number; total_centesimas: number | null; criterios: Credito[] };
}

interface Evaluacion {
  evaluacion_id: string;
  version_rubrica: string;
  rubrica_sha256: string;
  revision_actual: Revision;
  evidencias: Evidencia[];
  publicacion: Publicacion | null;
}

const mensaje = (error: unknown): string => error instanceof Error ? error.message : "No se pudo cargar la revisión.";
const formato = new Intl.NumberFormat("es-CL", { maximumFractionDigits: 2 });

export async function mostrarSupervision(main: HTMLElement, vigente: () => boolean): Promise<void> {
  if (!vigente()) return;
  history.replaceState(null, "", "#supervision");
  main.dataset.view = "supervision";
  main.innerHTML = `
    <section class="view" aria-labelledby="page-title">
      <div class="page-heading"><div class="heading-copy"><h1 id="page-title" tabindex="-1">Supervisión</h1></div></div>
      <div class="panel stack" data-listado>
        <div class="field"><label for="actividad-docente">Actividad</label><select id="actividad-docente"></select></div>
        <div class="table-wrap"><table>
          <thead><tr><th>Estudiante</th><th>Caso e intento</th><th>Estado</th><th>Acción</th></tr></thead>
          <tbody></tbody>
        </table></div>
        <p role="status" data-aviso></p>
        <details data-resultados>
          <summary>Resultados educativos de la actividad</summary>
          <div data-informe class="stack"></div>
        </details>
      </div>
    </section>`;
  const raiz = main.querySelector<HTMLElement>("[data-listado]")!;
  const selector = raiz.querySelector<HTMLSelectElement>("select")!;
  const aviso = raiz.querySelector<HTMLElement>("[data-aviso]")!;
  const cuerpo = raiz.querySelector("tbody")!;
  const resultados = raiz.querySelector<HTMLDetailsElement>("[data-resultados]")!;
  const informe = raiz.querySelector<HTMLElement>("[data-informe]")!;
  let cargaActual = 0;
  let actividades: Actividad[];
  try {
    actividades = await api<Actividad[]>("/docente/actividades");
  } catch (error) {
    if (vigente() && raiz.isConnected) aviso.textContent = mensaje(error);
    return;
  }
  if (!vigente() || !raiz.isConnected) return;
  selector.replaceChildren(...actividades.map((a) => new Option(a.nombre, a.actividad_id)));
  selector.addEventListener("change", () => {
    resultados.open = false;
    informe.replaceChildren();
    void cargar();
  });
  resultados.addEventListener("toggle", () => {
    if (resultados.open && selector.value) {
      const panel = document.createElement("div");
      panel.className = "stack";
      informe.replaceChildren(panel);
      void mostrarInforme(panel, selector.value);
    }
  });
  main.querySelector<HTMLElement>("#page-title")?.focus();
  if (actividades.length) await cargar();
  else aviso.textContent = "Sin actividades asignadas.";

  async function cargar(): Promise<void> {
    const elegida = selector.value;
    const actividad = selector.selectedOptions[0]?.textContent ?? "";
    const carga = ++cargaActual;
    const sigueAbierta = () => vigente() && raiz.isConnected && carga === cargaActual;
    cuerpo.replaceChildren();
    aviso.textContent = "";
    try {
      const lista = await api<Entrega[]>(`/actividades/${encodeURIComponent(elegida)}/entregas`);
      if (!sigueAbierta()) return;
      for (const entrega of lista) {
        const fila = cuerpo.insertRow();
        fila.insertCell().textContent = entrega.estudiante_id;
        fila.insertCell().textContent = `${actividad} · ${entrega.intento_id}`;
        fila.insertCell().textContent = entrega.evaluacion_id ? "Para revisar" : "Evaluación pendiente";
        const acciones = fila.insertCell();
        if (entrega.evaluacion_id) {
          const id = entrega.evaluacion_id;
          const boton = document.createElement("button");
          boton.className = "button small";
          boton.type = "button";
          boton.textContent = "Revisar";
          boton.addEventListener("click", async () => {
            boton.disabled = true;
            try {
              const evaluacion = await api<Evaluacion>("/evaluaciones/" + encodeURIComponent(id));
              if (sigueAbierta()) mostrarRevision(main, entrega, evaluacion, vigente);
            } catch (error) {
              if (sigueAbierta()) aviso.textContent = mensaje(error);
            } finally {
              boton.disabled = false;
            }
          });
          acciones.append(boton);
        }
      }
      if (!lista.length) aviso.textContent = "Aún no hay entregas confirmadas.";
    } catch (error) {
      if (sigueAbierta()) aviso.textContent = mensaje(error);
    }
  }
}

function mostrarRevision(
  main: HTMLElement, entrega: Entrega, evaluacion: Evaluacion, vigente: () => boolean, guardada = false,
): void {
  main.dataset.view = "revision";
  const revision = evaluacion.revision_actual;
  main.innerHTML = `
    <section class="view" aria-labelledby="page-title">
      <div class="page-heading"><div class="heading-copy"><h1 id="page-title" tabindex="-1">Revisión docente</h1></div></div>
      <div class="panel stack">
        <div class="button-row between">
          <div class="stack tight"><h2 data-estudiante></h2><p class="muted" data-datos></p></div>
          <button type="button" class="button" data-volver>${iconoValoracion("vista-volver", 20)}<span>Volver al grupo</span></button>
        </div>
        <div class="split">
          <div class="stack" data-evidencias><h3>Evidencia del intento</h3></div>
          <form class="stack">
            <h3>Devolución</h3>
            <p data-total></p>
            <div class="stack" data-criterios></div>
            <div class="field"><label for="motivo-rectificacion">Motivo de rectificación</label>
              <textarea id="motivo-rectificacion" rows="3" maxlength="1000" required></textarea></div>
            <div class="button-row"><button type="submit" class="button primary">Guardar revisión</button></div>
            <p role="status" data-aviso></p>
          </form>
        </div>
      </div>
    </section>`;
  const ruta = "/evaluaciones/" + encodeURIComponent(evaluacion.evaluacion_id);
  const evidencia = main.querySelector("[data-evidencias]")!;
  for (const item of evaluacion.evidencias) {
    const ficha = document.createElement("article");
    ficha.className = "tile stack tight";
    const tipo = document.createElement("h4");
    tipo.textContent = item.tipo;
    const contenido = document.createElement("p");
    contenido.textContent = item.resumen;
    const referencia = document.createElement("small");
    referencia.textContent = item.id;
    ficha.append(tipo, contenido, referencia);
    evidencia.append(ficha);
  }
  const total = revision.puntaje.total_centesimas;
  main.querySelector("[data-total]")!.textContent = total === null ? "Sin puntaje definitivo."
    : `Puntaje ${formato.format(total / 100)} de 100.`;
  main.querySelector("[data-estudiante]")!.textContent = entrega.estudiante_id;
  main.querySelector("[data-datos]")!.textContent = `Rúbrica V${evaluacion.version_rubrica} · Revisión ${revision.revision}`;
  const huella = document.createElement("p");
  huella.className = "small-text muted preserve-lines";
  huella.textContent = "SHA-256 de la rúbrica · " + evaluacion.rubrica_sha256;
  main.querySelector("[data-datos]")!.after(huella);
  if (revision.autor_id) {
    const historial = document.createElement("p");
    historial.className = "small-text muted";
    historial.textContent = `${revision.autor_id} · ${revision.motivo}`;
    main.querySelector("[data-datos]")!.after(historial);
  }
  if (revision.resultado.prioridad) {
    const prioridad = document.createElement("p");
    prioridad.textContent = `${revision.resultado.prioridad.accion} ${revision.resultado.prioridad.motivo}`;
    main.querySelector("[data-total]")!.after(prioridad);
  }
  const criterios = main.querySelector("[data-criterios]")!;
  const editores = revision.resultado.observaciones.map((obs, indice) => {
    const nota = revision.puntaje.criterios.find((c) => c.criterio_id === obs.criterio_id)!;
    const leer = editarCriterio(criterios, obs, nota, indice, evaluacion.evidencias, revision.puntaje.peso_aplicable);
    const traza = document.createElement("a");
    traza.href = `/api${ruta}/criterios/${encodeURIComponent(obs.criterio_id)}/traza?revision=${revision.revision}`;
    traza.target = "_blank";
    traza.rel = "noopener";
    traza.textContent = "Consultar traza de " + tituloCriterio(obs.criterio_id).toLocaleLowerCase("es-CL");
    criterios.lastElementChild?.append(traza);
    return leer;
  });
  const formulario = main.querySelector("form")!;
  const motivo = main.querySelector<HTMLTextAreaElement>("#motivo-rectificacion")!;
  const boton = formulario.querySelector<HTMLButtonElement>("button[type=submit]")!;
  const aviso = formulario.querySelector<HTMLElement>("[data-aviso]")!;
  montarPublicacion(formulario.querySelector(".button-row")!, ruta, revision, evaluacion.publicacion);
  let pendiente: { serializado: string; clave: string } | undefined;
  if (guardada) aviso.textContent = "Revisión guardada.";
  main.querySelector("[data-volver]")!.addEventListener("click", () => { void mostrarSupervision(main, vigente); });
  main.querySelector<HTMLElement>("#page-title")?.focus();
  formulario.addEventListener("submit", async (evento) => {
    evento.preventDefault();
    const cambios = editores.flatMap((leer) => leer() ?? []);
    if (!cambios.length) {
      aviso.textContent = "Modifica al menos un criterio.";
      return;
    }
    const peticion = { revision_esperada: revision.revision, motivo: motivo.value.trim(), cambios };
    const serializado = JSON.stringify(peticion);
    if (pendiente?.serializado !== serializado) pendiente = { serializado, clave: crypto.randomUUID() };
    boton.disabled = true;
    try {
      await api<Revision>(ruta, escribir("PATCH", peticion, pendiente.clave));
      if (vigente() && formulario.isConnected) {
        const actual = await api<Evaluacion>(ruta);
        if (vigente() && formulario.isConnected) mostrarRevision(main, entrega, actual, vigente, true);
      }
    } catch (error) {
      aviso.textContent = mensaje(error);
    } finally {
      boton.disabled = false;
    }
  });
}

function editarCriterio(
  criterios: Element, obs: Observacion, nota: Credito, indice: number, fuentes: Evidencia[], pesoTotal: number,
) {
  const id = `criterio-${indice}`;
  const fila = document.createElement("details");
  fila.className = "disclosure";
  fila.innerHTML = `
    <summary></summary>
    <fieldset class="stack">
      <legend></legend>
      <p data-expectativa></p>
      <label for="${id}-juicio">Juicio</label><select id="${id}-juicio"></select>
      <label for="${id}-credito">Crédito (%)</label><input id="${id}-credito" type="number" min="0" max="100" step="1">
      <label for="${id}-actuacion">Actuación</label><textarea id="${id}-actuacion" maxlength="1400" rows="2"></textarea>
      <label for="${id}-explicacion">Explicación</label><textarea id="${id}-explicacion" maxlength="1400" rows="2"></textarea>
      <label for="${id}-mejora">Mejora</label><textarea id="${id}-mejora" maxlength="1400" rows="2"></textarea>
      <fieldset class="stack tight" data-citas><legend>Citas del intento</legend></fieldset>
    </fieldset>`;
  const etiqueta = tituloCriterio(obs.criterio_id);
  const aporte = nota.puntos_ponderados === null || pesoTotal === 0 ? "" : ` · ${formato.format(nota.puntos_ponderados / pesoTotal)} puntos`;
  fila.querySelector("summary")!.textContent = `${etiqueta} · ${tituloJuicio(obs.juicio)} · ${nota.peso} %${aporte}`;
  fila.querySelector("legend")!.textContent = etiqueta;
  fila.querySelector("[data-expectativa]")!.textContent = obs.expectativa;
  const selector = fila.querySelector("select")!;
  selector.replaceChildren(...Object.entries(JUICIOS).map(([valor, texto]) => new Option(texto, valor)));
  selector.value = obs.juicio;
  const credito = fila.querySelector<HTMLInputElement>("input[type=number]")!;
  if (nota.credito !== null) credito.value = String(nota.credito);
  function estadoCredito(): void {
    credito.disabled = ["no_evaluable", "revision_docente"].includes(selector.value);
    if (credito.disabled) credito.value = "";
    else if (!credito.value) credito.value = selector.value === "acierto" ? "100" : "0";
  }
  estadoCredito();
  selector.addEventListener("change", estadoCredito);
  const [actuacion, explicacion, mejora] = fila.querySelectorAll("textarea");
  actuacion.value = obs.actuacion;
  explicacion.value = obs.explicacion;
  mejora.value = obs.mejora;
  const citas = fila.querySelector("[data-citas]")!;
  const seleccionados = new Set(obs.evidencias);
  for (const fuente of fuentes) {
    const etiqueta = document.createElement("label");
    const casilla = document.createElement("input");
    casilla.type = "checkbox";
    casilla.value = fuente.id;
    casilla.checked = seleccionados.has(fuente.id);
    etiqueta.append(casilla, ` ${fuente.tipo} · ${fuente.resumen}`);
    citas.append(etiqueta);
  }
  if (obs.incidencia) {
    const aviso = document.createElement("p");
    aviso.textContent = "Un acuse carece de evento recuperable. Revisa su origen antes de resolver el criterio.";
    fila.querySelector("[data-expectativa]")!.after(aviso);
  }
  criterios.append(fila);
  const leer = () => ({
    criterio_id: obs.criterio_id, juicio: selector.value, credito: credito.disabled ? null : Number(credito.value),
    actuacion: actuacion.value.trim(), explicacion: explicacion.value.trim(), mejora: mejora.value.trim(),
    evidencias: Array.from(citas.querySelectorAll<HTMLInputElement>("input:checked"), (control) => control.value).sort(),
  });
  const inicial = JSON.stringify(leer());
  return () => {
    const actual = leer();
    return JSON.stringify(actual) === inicial ? undefined : actual;
  };
}
