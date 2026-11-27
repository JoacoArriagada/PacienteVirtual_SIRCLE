import { api, ErrorHttp, escribir } from "../../app/api.js";
import { mostrarDato, type Evidencia } from "./apuntes.js";

interface Referencia {
  tipo: string;
  origen_id: string;
}

interface Hipotesis {
  rol: "principal" | "alternativa";
  titulo: string;
  explicacion: string;
  evidencias: Referencia[];
}

export interface Borrador {
  revision: number;
  hipotesis: Hipotesis[];
}

interface Registros {
  revision: number;
  borrador?: Borrador;
  evidencias?: Evidencia[];
}

interface Fila {
  rol: Hipotesis["rol"];
  titulo: HTMLInputElement;
  explicacion: HTMLTextAreaElement;
  vinculos: HTMLFieldSetElement;
}

const CLAVE = (ref: Referencia) => JSON.stringify([ref.tipo, ref.origen_id]);
const VACIA = (rol: Hipotesis["rol"]): Hipotesis => ({ rol, titulo: "", explicacion: "", evidencias: [] });

export function montarHipotesis(panel: HTMLElement, intento: string, registros: Registros) {
  const ruta = "/intentos/" + encodeURIComponent(intento);
  panel.innerHTML = `
    <h2>Tu hipótesis</h2>
    <div class="stack tight notebook-scroll" data-resumen-hipotesis></div>
    <button class="button primary small" type="button" data-abrir-hipotesis>Fundamentar hipótesis</button>
    <form class="stack notebook-scroll" hidden>
      <button class="text-button notebook-back" type="button" data-volver-hipotesis>Volver a hipótesis</button>
      <div class="stack" data-hipotesis></div>
      <div class="button-row">
        <button class="button" type="button" data-agregar>Agregar alternativa</button>
        <button class="button primary" type="submit">Guardar hipótesis</button>
        <button class="button" type="button" data-actualizar>Actualizar evidencias</button>
        <button class="button" type="button" data-cargar hidden>Cargar versión guardada</button>
      </div>
      <p role="status" data-estado hidden></p>
    </form>`;
  const resumen = panel.querySelector<HTMLElement>("[data-resumen-hipotesis]")!;
  const editar = panel.querySelector<HTMLButtonElement>("[data-abrir-hipotesis]")!;
  const formulario = panel.querySelector("form")!;
  const volver = formulario.querySelector<HTMLButtonElement>("[data-volver-hipotesis]")!;
  const lista = formulario.querySelector<HTMLElement>("[data-hipotesis]")!;
  const agregar = formulario.querySelector<HTMLButtonElement>("[data-agregar]")!;
  const cargar = formulario.querySelector<HTMLButtonElement>("[data-cargar]")!;
  const guardar = formulario.querySelector<HTMLButtonElement>('button[type="submit"]')!;
  const estado = formulario.querySelector<HTMLElement>("[data-estado]")!;
  const filas: Fila[] = [];
  let disponibles: Evidencia[] = [];
  let revision = registros.revision;
  let borrador = registros.borrador;
  let pendiente: { cuerpo: string; clave: string } | undefined;
  let ocupada = false;
  let modificada = false;

  function avisar(texto: string): void {
    estado.textContent = texto;
    estado.hidden = false;
  }

  function seleccion(campo: HTMLFieldSetElement): Referencia[] {
    return Array.from(campo.querySelectorAll<HTMLInputElement>("input:checked"), (opcion) => ({
      tipo: opcion.dataset.tipo!,
      origen_id: opcion.value,
    }));
  }

  function pintarVinculos(campo: HTMLFieldSetElement, referencias: Referencia[]): void {
    const usadas = new Set(referencias.map(CLAVE));
    const leyenda = document.createElement("legend");
    leyenda.textContent = "Vincular evidencias";
    campo.replaceChildren(leyenda);
    for (const dato of disponibles) {
      const etiqueta = document.createElement("label");
      etiqueta.className = "check";
      const opcion = document.createElement("input");
      opcion.type = "checkbox";
      opcion.value = dato.origen_id;
      opcion.dataset.tipo = dato.tipo;
      opcion.checked = usadas.has(CLAVE(dato));
      const texto = document.createElement("span");
      texto.textContent = [dato.origen, mostrarDato(dato), dato.origen_id].join(" · ");
      etiqueta.append(opcion, texto);
      campo.append(etiqueta);
    }
    if (!disponibles.length) {
      const mensaje = document.createElement("p");
      mensaje.textContent = "Aún no hay registros para vincular.";
      campo.append(mensaje);
    }
  }

  function pintarFila(hipotesis: Hipotesis): HTMLInputElement {
    const numero = filas.length + 1;
    const seccion = document.createElement("fieldset");
    seccion.className = "stack tight";
    const rol = hipotesis.rol === "principal" ? "Hipótesis principal" : "Alternativa";
    seccion.innerHTML = `
      <legend>${rol}</legend>
      <label for="hipotesis-${numero}">Interpretación provisional</label>
      <input id="hipotesis-${numero}" type="text" maxlength="180">
      <label for="explicacion-${numero}">Fundamento y límites</label>
      <textarea id="explicacion-${numero}" rows="3" maxlength="1400"></textarea>
      <fieldset class="compact-scroll"></fieldset>`;
    const titulo = seccion.querySelector("input")!;
    const explicacion = seccion.querySelector("textarea")!;
    const vinculos = seccion.querySelector("fieldset")!;
    titulo.value = hipotesis.titulo;
    explicacion.value = hipotesis.explicacion;
    pintarVinculos(vinculos, hipotesis.evidencias);
    filas.push({ rol: hipotesis.rol, titulo, explicacion, vinculos });
    lista.append(seccion);
    agregar.disabled = filas.length >= 12;
    return titulo;
  }

  function pintar(actual: Registros): void {
    revision = actual.revision;
    borrador = actual.borrador;
    disponibles = (actual.evidencias ?? []).filter((dato) => dato.tipo !== "orientacion");
    filas.length = 0;
    lista.replaceChildren();
    const guardadas = borrador?.hipotesis ?? [];
    pintarFila(guardadas.find((item) => item.rol === "principal") ?? VACIA("principal"));
    const alternativas = guardadas.filter((item) => item.rol === "alternativa");
    for (const item of alternativas.length ? alternativas : [VACIA("alternativa")]) pintarFila(item);
    pintarResumen();
  }

  function pintarResumen(): void {
    const guardadas = borrador?.hipotesis ?? [];
    editar.textContent = guardadas.length ? "Editar hipótesis" : "Fundamentar hipótesis";
    resumen.replaceChildren();
    if (!guardadas.length) {
      resumen.innerHTML = `<div class="empty">
        <strong>Sin hipótesis guardada</strong>
        Relaciona tu interpretación con evidencia obtenida y anota qué falta por explorar.
      </div>`;
    }
    for (const item of guardadas) {
      const tarjeta = document.createElement("article");
      tarjeta.className = "tile stack tight";
      tarjeta.innerHTML = '<p class="source-label"></p><h3></h3><p class="preserve-lines small-text"></p><p class="source-label"></p>';
      tarjeta.querySelector("p")!.textContent = item.rol === "principal" ? "Hipótesis principal" : "Alternativa";
      tarjeta.querySelector("h3")!.textContent = item.titulo;
      tarjeta.querySelector(".preserve-lines")!.textContent = item.explicacion;
      const cantidad = item.evidencias.length;
      tarjeta.lastElementChild!.textContent = cantidad === 1 ? "1 evidencia vinculada" : `${cantidad} evidencias vinculadas`;
      resumen.append(tarjeta);
    }
  }

  async function actualizar(cargarVersion: boolean): Promise<void> {
    if (ocupada) return;
    ocupada = true;
    guardar.disabled = true;
    try {
      const actual = await api<Registros>(ruta);
      if (cargarVersion || !modificada) {
        pintar(actual);
        modificada = false;
        pendiente = undefined;
        cargar.hidden = true;
        if (cargarVersion) avisar("Versión guardada cargada.");
      } else if (actual.borrador?.revision === borrador?.revision) {
        revision = actual.revision;
        disponibles = (actual.evidencias ?? []).filter((dato) => dato.tipo !== "orientacion");
        for (const fila of filas) pintarVinculos(fila.vinculos, seleccion(fila.vinculos));
      } else {
        cargar.hidden = false;
        avisar("Hay un borrador guardado más reciente. Puedes cargarlo desde este formulario.");
      }
    } catch (error) {
      avisar(error instanceof Error ? error.message : "No se pudieron recuperar las hipótesis.");
    } finally {
      guardar.disabled = false;
      ocupada = false;
    }
  }

  function abrir(): void {
    resumen.hidden = editar.hidden = true;
    formulario.hidden = false;
    volver.focus();
    void actualizar(false);
  }

  editar.addEventListener("click", abrir);
  volver.addEventListener("click", () => {
    formulario.hidden = true;
    resumen.hidden = editar.hidden = false;
    editar.focus();
  });
  agregar.addEventListener("click", () => {
    modificada = true;
    pintarFila(VACIA("alternativa")).focus();
  });
  formulario.addEventListener("input", () => { modificada = true; });
  formulario.querySelector("[data-actualizar]")!.addEventListener("click", () => { void actualizar(false); });
  cargar.addEventListener("click", () => { void actualizar(true); });
  formulario.addEventListener("submit", async (evento) => {
    evento.preventDefault();
    if (ocupada) return;
    const hipotesis = filas.map(({ rol, titulo, explicacion, vinculos }) => ({
      rol,
      titulo: titulo.value.trim(),
      explicacion: explicacion.value.trim(),
      evidencias: seleccion(vinculos),
    })).filter((item) => item.titulo || item.explicacion || item.evidencias.length);
    const cuerpo = { revision_esperada: revision, hipotesis };
    const serializado = JSON.stringify(cuerpo);
    if (pendiente?.cuerpo !== serializado) pendiente = { cuerpo: serializado, clave: crypto.randomUUID() };
    ocupada = true;
    const controles = Array.from(formulario.elements).filter((elemento) =>
      elemento instanceof HTMLInputElement || elemento instanceof HTMLButtonElement || elemento instanceof HTMLTextAreaElement);
    const bloqueados = controles.filter((control) => !control.disabled);
    bloqueados.forEach((control) => { control.disabled = true; });
    try {
      borrador = await api<Borrador>(ruta + "/razonamiento", escribir("PUT", cuerpo, pendiente.clave));
      revision = borrador.revision;
      pendiente = undefined;
      modificada = false;
      cargar.hidden = true;
      pintarResumen();
      avisar("Hipótesis guardadas.");
    } catch (error) {
      if (error instanceof ErrorHttp && error.estado === 409) cargar.hidden = false;
      avisar(error instanceof Error ? error.message : "No se pudo guardar el borrador.");
    } finally {
      bloqueados.forEach((control) => { control.disabled = false; });
      ocupada = false;
    }
  });
  pintar(registros);
  return { abrir, cambiosSinGuardar: () => modificada };
}
