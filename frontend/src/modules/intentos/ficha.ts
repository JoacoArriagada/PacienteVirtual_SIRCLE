import { api } from "../../app/api.js";
import { abrirDialogo } from "../../app/dialogo.js";
import { montarChat, type Turno } from "../entrevista/chat.js";
import type { Sugerencia } from "../entrevista/sugerencias.js";
import { montarExtremidad, retratoLR } from "../valoracion/brazo.js";
import { mostrarPreparacionBrazalete } from "../valoracion/brazalete.js";
import { mostrarInstrumento, textoMedicion, type Medicion } from "../valoracion/instrumento.js";
import { pulso } from "../valoracion/pulso.js";
import { peso } from "../valoracion/peso.js";
import { respiracion } from "../valoracion/respiracion.js";
import { saturacion } from "../valoracion/saturacion.js";
import { temperatura } from "../valoracion/temperatura.js";
import { mostrarExploracion } from "../valoracion/exploracion.js";
import { inspeccion } from "../valoracion/inspeccion.js";
import { palpacion } from "../valoracion/palpacion.js";
import { auscultacion } from "../valoracion/auscultacion.js";
import type { Orden } from "../examenes/ecg.js";
import { montarAdicionales } from "../examenes/adicionales.js";
import { montarApuntes, mostrarDato, type Apunte, type Evidencia } from "../razonamiento/apuntes.js";
import { montarEvidencias } from "../razonamiento/hallazgos.js";
import { montarHipotesis, type Borrador } from "../razonamiento/hipotesis.js";
import { montarEntrega, mostrarAcuse, type AcuseEntrega } from "../razonamiento/entrega.js";
import { mostrarPre } from "../encuestas/pre.js";
import { montarPestanas } from "./pestanas.js";

export interface VistaIntento {
  intento_id: string;
  actividad_id: string;
  version_caso: string;
  pre: string;
  post: string;
  estado: string;
  fase: string;
  revision: number;
  rubrica_id?: number;
  entrega?: AcuseEntrega;
  introduccion?: { paciente: string; edad: number; motivo: string; contexto: string };
  ficha?: { fuente: string; fase: string; datos: { etiqueta: string; valor: string }[]; hallazgos?: Medicion[] };
  indicaciones?: string[];
  ayuda?: { origen: string; texto: string };
  sugerencias?: Sugerencia[];
  turnos?: Turno[];
  ordenes?: Orden[];
  apuntes?: Apunte[];
  evidencias?: Evidencia[];
  borrador?: Borrador;
  respiraciones_simuladas?: number | null;
}

type VistaClinica = VistaIntento & Required<Pick<VistaIntento, "introduccion" | "ficha" | "indicaciones" | "ayuda">>;

function tieneFicha(intento: VistaIntento): intento is VistaClinica {
  return Boolean(intento.introduccion && intento.ficha && intento.indicaciones && intento.ayuda);
}

export function mostrarFicha(main: HTMLElement, intento: VistaIntento, volver: () => void, vigente: () => boolean): void {
  if (!vigente()) return;
  const enAtencion = intento.pre !== "pendiente" && intento.estado === "atencion" && tieneFicha(intento);
  main.dataset.view = "intento";
  main.innerHTML = `
    <section class="view${enAtencion ? " atencion-view" : ""}" aria-labelledby="page-title">
      <div class="page-heading with-back">
        <div class="heading-back"><button class="${enAtencion ? "text-button" : "button"}" type="button" data-volver>
          ${enAtencion ? '<span aria-hidden="true">←</span> ' : ""}Volver a mis pacientes
        </button></div>
        <div class="heading-copy"><h1 id="page-title" tabindex="-1">${enAtencion ? "Atender y conversar" : "Práctica de LR"}</h1></div>
      </div>
      <div class="panel" data-contenido-intento></div>
    </section>`;
  main.querySelector("[data-volver]")!.addEventListener("click", volver);
  const vista = main.querySelector<HTMLElement>(".view")!;
  const panel = main.querySelector<HTMLElement>("[data-contenido-intento]")!;
  if (intento.pre === "pendiente") {
    mostrarPre(panel, intento.intento_id, (actual) => mostrarFicha(main, actual, volver, vigente));
  } else if (enAtencion) {
    montarAtencion(main, vista, panel, intento, volver, vigente);
  } else if (tieneFicha(intento)) {
    montarFichaInicial(panel, intento);
  } else {
    panel.textContent = "La ficha no está disponible en esta fase.";
  }
  if (intento.entrega) mostrarAcuse(vista, intento.entrega, intento);
  main.querySelector<HTMLElement>("#page-title")?.focus();
}

function montarAtencion(
  main: HTMLElement, vista: HTMLElement, panel: HTMLElement, intento: VistaClinica,
  volver: () => void, vigente: () => boolean,
): void {
  const id = intento.intento_id;
  const ruta = "/intentos/" + encodeURIComponent(id);
  const { introduccion } = intento;
  panel.classList.add("session-frame");
  const plantilla = document.querySelector<HTMLTemplateElement>("#plantilla-atencion")!;
  panel.append(plantilla.content.cloneNode(true));
  const buscar = (selector: string) => panel.querySelector<HTMLElement>(selector)!;
  buscar("[data-paciente]").textContent = introduccion.paciente;
  buscar("[data-edad]").textContent = `${introduccion.edad} años`;
  montarPestanas(buscar("[data-pestanas-examenes]"));
  const elegirCuaderno = montarPestanas(buscar("[data-pestanas-cuaderno]"), (seleccion) => {
    if (seleccion === "cuaderno-evidencia") void recargarEvidencias();
  });
  const grid = buscar(".clinic-grid");
  const paneles = Array.from(panel.querySelectorAll<HTMLButtonElement>("[data-seccion]"));
  function cambiarPanel(seccion: string): void {
    grid.dataset.mobile = seccion;
    for (const boton of paneles) boton.setAttribute("aria-pressed", String(boton.dataset.seccion === seccion));
  }
  for (const boton of paneles) boton.addEventListener("click", () => cambiarPanel(boton.dataset.seccion!));
  buscar("[data-ir-cuaderno]").addEventListener("click", () => {
    cambiarPanel("notebook");
    elegirCuaderno("cuaderno-chat", true);
  });

  const volverAtencion = async () => {
    const procedimiento = main.firstElementChild;
    const sigueAbierto = () => vigente() && main.firstElementChild === procedimiento;
    try {
      const actual = await api<VistaIntento>(ruta);
      if (sigueAbierto()) mostrarFicha(main, actual, volver, vigente);
    } catch {
      if (sigueAbierto()) volver();
    }
  };
  const ciclos = intento.respiraciones_simuladas;
  const instrumentos = [pulso, temperatura, respiracion(ciclos ?? 0), saturacion, peso];
  montarExtremidad(buscar("#panel-general"), () => mostrarPreparacionBrazalete(main, id, volverAtencion), [
    ...instrumentos.map((config) => ({
      ...config,
      etiqueta: config === pulso ? "Signos vitales" : undefined,
      abrir: config.id === "respiracion" && !ciclos ? undefined : () => mostrarInstrumento(main, id, config, volverAtencion),
    })),
    ...[inspeccion, palpacion, auscultacion].map((config) => ({
      ...config, exploracion: true,
      abrir: () => mostrarExploracion(main, id, config, volverAtencion),
    })),
  ]);

  function abrirFicha(): void {
    const contenido = document.createElement("div");
    contenido.className = "stack";
    contenido.innerHTML = '<div class="stack tight"><h3></h3><p></p></div>';
    contenido.querySelector("h3")!.textContent = introduccion.motivo;
    contenido.querySelector("p")!.textContent = introduccion.contexto;
    contenido.append(registrosFicha(intento.ficha));
    abrirDialogo("Ficha clínica", contenido);
  }
  buscar("[data-abrir-ficha]").addEventListener("click", abrirFicha);
  buscar("[data-abrir-evolucion]").addEventListener("click", () => { void abrirEvolucion(intento, ruta); });
  buscar("[data-abrir-indicaciones]").addEventListener("click", () => abrirDialogo("Indicaciones", listaTextos(intento.indicaciones)));
  buscar("[data-abrir-ayuda]").addEventListener("click", () => abrirDialogo("Ayuda en la atención", orientacion(intento.ayuda)));

  const panelEvidencia = buscar("#cuaderno-evidencia");
  const actualizarSiVisible = () => {
    if (!panelEvidencia.hidden && panelEvidencia.isConnected) void recargarEvidencias();
  };
  const irAlTurno = montarChat(buscar("#cuaderno-chat"), id, intento.turnos ?? [], intento.sugerencias ?? [], introduccion, actualizarSiVisible);
  const hipotesis = montarHipotesis(buscar("#cuaderno-hipotesis"), id, intento);
  montarApuntes(buscar("#cuaderno-notas"), id, intento, () => elegirCuaderno("cuaderno-evidencia", true), actualizarSiVisible);
  const recargarEvidencias = montarEvidencias(panelEvidencia, id, introduccion.motivo, intento.evidencias ?? [],
    () => elegirCuaderno("cuaderno-chat", true),
    () => { elegirCuaderno("cuaderno-hipotesis"); hipotesis.abrir(); },
    abrirFicha,
    (turno) => { elegirCuaderno("cuaderno-chat", true); irAlTurno(turno); });
  montarAdicionales(buscar("#panel-adicionales"), id, intento.ordenes ?? [], actualizarSiVisible);
  if (intento.rubrica_id) {
    montarEntrega(buscar("[data-revisar-entrega]"), vista, id, hipotesis.cambiosSinGuardar,
      (actual) => mostrarFicha(main, actual, volver, vigente));
  }
}

function montarFichaInicial(panel: HTMLElement, intento: VistaClinica): void {
  panel.classList.add("intro-grid");
  panel.innerHTML = `
    <img class="portrait" src="${retratoLR}" alt="Retrato de LR">
    <div class="intro-content">
      <div class="patient-name-line"><h2 data-paciente></h2><span class="age" data-edad></span></div>
      <h3 data-motivo></h3><p data-contexto></p>
      <details class="disclosure" data-ficha-inicial><summary>Ficha inicial</summary></details>
      <details class="disclosure" data-indicaciones><summary>Indicaciones</summary></details>
      <details class="disclosure" data-orientacion><summary>Ayuda</summary></details>
    </div>`;
  panel.querySelector("[data-paciente]")!.textContent = intento.introduccion.paciente;
  panel.querySelector("[data-edad]")!.textContent = `${intento.introduccion.edad} años`;
  panel.querySelector("[data-motivo]")!.textContent = intento.introduccion.motivo;
  panel.querySelector("[data-contexto]")!.textContent = intento.introduccion.contexto;
  panel.querySelector("[data-ficha-inicial]")!.append(registrosFicha(intento.ficha));
  panel.querySelector("[data-indicaciones]")!.append(listaTextos(intento.indicaciones));
  panel.querySelector("[data-orientacion]")!.append(orientacion(intento.ayuda));
}

function registrosFicha(ficha: VistaClinica["ficha"]): HTMLElement {
  const contenido = document.createElement("div");
  contenido.className = "stack tight";
  contenido.innerHTML = '<dl class="pairs" data-ficha></dl><p class="muted small-text" data-fuente></p>';
  contenido.querySelector("[data-fuente]")!.textContent = ficha.fuente;
  const detalles = contenido.querySelector("dl")!;
  for (const dato of ficha.datos) {
    const termino = document.createElement("dt");
    termino.textContent = dato.etiqueta;
    const valor = document.createElement("dd");
    valor.textContent = dato.valor;
    detalles.append(termino, valor);
  }
  if (ficha.hallazgos?.length) {
    const titulo = document.createElement("h3");
    titulo.textContent = "Mediciones registradas";
    const lista = document.createElement("ul");
    lista.dataset.hallazgos = "";
    for (const medicion of ficha.hallazgos) {
      const fila = document.createElement("li");
      fila.dataset.medicionId = medicion.medicion_id;
      fila.textContent = [textoMedicion(medicion), medicion.origen, medicion.momento].filter(Boolean).join(" · ");
      lista.append(fila);
    }
    contenido.append(titulo, lista);
  }
  return contenido;
}

function listaTextos(textos: string[]): HTMLUListElement {
  const lista = document.createElement("ul");
  for (const texto of textos) {
    const fila = document.createElement("li");
    fila.textContent = texto;
    lista.append(fila);
  }
  return lista;
}

function orientacion(ayuda: VistaClinica["ayuda"]): HTMLElement {
  const contenido = document.createElement("div");
  contenido.className = "inset";
  contenido.innerHTML = '<h3>Orientación docente</h3><p data-ayuda></p>';
  const texto = contenido.querySelector("p")!;
  texto.textContent = ayuda.texto;
  texto.dataset.origen = ayuda.origen;
  return contenido;
}

async function abrirEvolucion(intento: VistaIntento, ruta: string): Promise<void> {
  const contenido = document.createElement("div");
  contenido.className = "stack";
  const registros = document.createElement("div");
  registros.className = "stack tight";
  const aviso = document.createElement("p");
  aviso.className = "small-text";
  aviso.setAttribute("role", "status");
  aviso.hidden = true;
  contenido.append(registros, aviso);

  function pintar(actual: VistaIntento): void {
    const fase = document.createElement("h3");
    fase.textContent = actual.fase === "inicial" ? "Valoración inicial" : `Fase ${actual.fase}`;
    const hallazgos = (actual.evidencias ?? []).filter(({ tipo }) => tipo !== "orientacion" && tipo !== "apunte");
    const ordenes = (actual.ordenes ?? []).map((orden) => {
      const nombre = orden.examen === "ecg-inicial" ? "Electrocardiograma inicial" : orden.examen;
      return ["Orden confirmada", nombre, orden.momento].filter(Boolean).join(" · ");
    });
    const entradas = hallazgos.map((dato) => [dato.origen, mostrarDato(dato)].filter(Boolean).join(" · "));
    registros.replaceChildren(fase);
    if (ordenes.length) registros.append(listaTextos(ordenes));
    if (entradas.length) registros.append(listaTextos(entradas));
    if (!entradas.length && !ordenes.length) {
      const mensaje = document.createElement("p");
      mensaje.className = "muted";
      mensaje.textContent = "Los hallazgos y órdenes de la atención aparecerán aquí.";
      registros.append(mensaje);
    }
  }

  pintar(intento);
  const dialogo = abrirDialogo("Evolución", contenido);
  try {
    const actual = await api<VistaIntento>(ruta);
    if (dialogo.open && contenido.isConnected) pintar(actual);
  } catch (error) {
    aviso.textContent = error instanceof Error ? error.message : "No se pudo actualizar la evolución.";
    aviso.hidden = false;
  }
}
