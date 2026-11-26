import { api } from "../../app/api.js";
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
import { montarEcg, type Orden } from "../examenes/ecg.js";
import { montarApuntes, type Apunte, type Evidencia } from "../razonamiento/apuntes.js";
import { montarHipotesis, type Borrador } from "../razonamiento/hipotesis.js";
import { montarEntrega, mostrarAcuse, type AcuseEntrega } from "../razonamiento/entrega.js";

export interface VistaIntento {
  intento_id: string;
  actividad_id: string;
  version_caso: string;
  pre: string;
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
  respiraciones_simuladas?: number;
}

export function mostrarFicha(main: HTMLElement, intento: VistaIntento, volver: () => void): void {
  main.dataset.view = "intento";
  main.innerHTML = `
    <section class="view" aria-labelledby="page-title">
      <div class="page-heading with-back">
        <div class="heading-back"><button class="button" type="button" data-volver>Volver a mis pacientes</button></div>
        <div class="heading-copy"><h1 id="page-title" tabindex="-1">Práctica de LR</h1></div>
      </div>
      <div class="panel" data-contenido-intento></div>
    </section>`;
  main.querySelector("[data-volver]")!.addEventListener("click", volver);
  const panel = main.querySelector<HTMLElement>("[data-contenido-intento]")!;
  const { introduccion, ficha, indicaciones, ayuda } = intento;
  if (intento.pre === "pendiente") {
    panel.textContent = "Pre pendiente.";
  } else if (introduccion && ficha && indicaciones && ayuda) {
    panel.classList.add("intro-grid");
    panel.innerHTML = `
      <img class="portrait" src="${retratoLR}" alt="Retrato de LR">
      <div class="intro-content">
        <div class="patient-name-line"><h2 data-paciente></h2><span class="age" data-edad></span></div>
        <div class="reason-line"><h3 data-motivo></h3></div>
        <p data-contexto></p>
        <details class="disclosure">
          <summary>Ficha inicial</summary>
          <dl class="pairs" data-ficha></dl>
          <p class="muted small-text" data-fuente></p>
          <ul data-hallazgos></ul>
        </details>
        <details class="disclosure">
          <summary>Indicaciones</summary>
          <ul data-indicaciones></ul>
        </details>
        <details class="disclosure">
          <summary>Ayuda</summary>
          <h3>Orientación docente</h3>
          <p data-ayuda data-origen></p>
        </details>
      </div>`;
    panel.querySelector("[data-paciente]")!.textContent = introduccion.paciente;
    panel.querySelector("[data-edad]")!.textContent = `${introduccion.edad} años`;
    panel.querySelector("[data-motivo]")!.textContent = introduccion.motivo;
    panel.querySelector("[data-contexto]")!.textContent = introduccion.contexto;
    panel.querySelector("[data-fuente]")!.textContent = ficha.fuente;
    const detalles = panel.querySelector("[data-ficha]")!;
    for (const dato of ficha.datos) {
      const termino = document.createElement("dt");
      termino.textContent = dato.etiqueta;
      const valor = document.createElement("dd");
      valor.textContent = dato.valor;
      detalles.append(termino, valor);
    }
    const registrados = panel.querySelector("[data-hallazgos]")!;
    for (const medicion of ficha.hallazgos ?? []) {
      const fila = document.createElement("li");
      fila.dataset.medicionId = medicion.medicion_id;
      fila.textContent = textoMedicion(medicion) + " · " + medicion.momento;
      registrados.append(fila);
    }
    const lista = panel.querySelector("[data-indicaciones]")!;
    for (const indicacion of indicaciones) {
      const li = document.createElement("li");
      li.textContent = indicacion;
      lista.append(li);
    }
    const orientacion = panel.querySelector<HTMLElement>("[data-ayuda]")!;
    orientacion.textContent = ayuda.texto;
    orientacion.dataset.origen = ayuda.origen;
    if (intento.estado === "atencion") {
      const atencion = document.createElement("div");
      atencion.className = "panel session-frame";
      atencion.innerHTML = `
        <div class="session-bar">
          <img src="${retratoLR}" class="portrait" alt="" width="44" height="44">
          <h2>Atender y conversar</h2>
          <span class="state" data-tone="warning">En curso</span>
        </div>
        <div class="clinic-grid">
          <section class="clinic-card clinic-actions" data-valoracion></section>
          <figure class="clinic-card clinic-portrait">
            <img src="${retratoLR}" class="portrait" alt="Paciente LR">
            <figcaption class="portrait-caption" data-pie-paciente></figcaption>
          </figure>
          <section class="clinic-card notebook" aria-label="Cuaderno de atención">
            <div class="notebook-panel" data-entrevista></div>
          </section>
        </div>`;
      panel.before(atencion);
      atencion.querySelector("[data-pie-paciente]")!.textContent = introduccion.motivo;
      const valoracion = atencion.querySelector<HTMLElement>("[data-valoracion]")!;
      const volverAtencion = async () => {
        try {
          const actual = await api<VistaIntento>("/intentos/" + encodeURIComponent(intento.intento_id));
          mostrarFicha(main, actual, volver);
        } catch {
          volver();
        }
      };
      const instrumentos = [pulso, temperatura, saturacion, peso];
      if (intento.respiraciones_simuladas) instrumentos.push(respiracion(intento.respiraciones_simuladas));
      montarExtremidad(valoracion, () => mostrarPreparacionBrazalete(main, intento.intento_id, volverAtencion),
        [
          ...instrumentos.map((instrumento) => ({
            ...instrumento,
            abrir: () => mostrarInstrumento(main, intento.intento_id, instrumento, volverAtencion),
          })),
          ...[inspeccion, palpacion, auscultacion].map((instrumento) => ({
            ...instrumento, exploracion: true,
            abrir: () => mostrarExploracion(main, intento.intento_id, instrumento, volverAtencion),
          })),
        ]);
      const entrevista = atencion.querySelector<HTMLElement>("[data-entrevista]")!;
      montarChat(entrevista, intento.intento_id, intento.turnos ?? [], intento.sugerencias ?? []);
      const examenes = document.createElement("section");
      main.querySelector(".view")!.append(examenes);
      montarEcg(examenes, intento.intento_id, intento.ordenes?.find((orden) => orden.examen === "ecg-inicial"));
      const razonamiento = document.createElement("section");
      main.querySelector(".view")!.append(razonamiento);
      montarApuntes(razonamiento, intento.intento_id, intento);
      const hipotesis = document.createElement("details");
      razonamiento.append(hipotesis);
      const cambiosSinGuardar = montarHipotesis(hipotesis, intento.intento_id, intento);
      if (intento.rubrica_id) {
        const acciones = document.createElement("div");
        acciones.className = "button-row";
        razonamiento.append(acciones);
        montarEntrega(acciones, main.querySelector<HTMLElement>(".view")!, intento.intento_id,
          cambiosSinGuardar, (actual) => mostrarFicha(main, actual, volver));
      }
    }
  } else {
    panel.textContent = "La ficha no está disponible en esta fase.";
  }
  if (intento.entrega) mostrarAcuse(main.querySelector<HTMLElement>(".view")!, intento.entrega, intento.intento_id);
  main.querySelector<HTMLElement>("#page-title")?.focus();
}
