import { api } from "../../app/api.js";
import { abrirDialogo } from "../../app/dialogo.js";
import { iconoValoracion } from "../valoracion/brazo.js";
import type { Medicion } from "../valoracion/instrumento.js";
import { resumenMedicion } from "../valoracion/resumen-medicion.js";
import { mostrarDato, type Evidencia } from "./apuntes.js";

interface Registros {
  evidencias?: Evidencia[];
  mediciones?: Medicion[];
}

const categorias: Record<string, string> = {
  declaracion: "Declaración",
  medicion: "Dato medido",
  observacion: "Observación",
  orientacion: "Orientación",
  informe: "Informe",
  interpretacion: "Interpretación",
};

function tarjetaEvidencia(dato: Evidencia, medicion?: Medicion, detalle = false): HTMLElement {
  const tarjeta = document.createElement(detalle ? "article" : "li");
  tarjeta.className = "evidence-item";
  tarjeta.dataset.origenId = dato.origen_id;

  if (dato.tipo === "medicion" && dato.instrumento && dato.valor && dato.unidad) {
    tarjeta.append(resumenMedicion({
      instrumento: dato.instrumento, valor: dato.valor, unidad: dato.unidad,
      origen: medicion?.origen ?? dato.origen, momento: dato.momento ?? medicion?.momento,
    }));
  } else {
    tarjeta.innerHTML = '<strong></strong><p class="small-text preserve-lines"></p><p class="source-label"></p>';
    tarjeta.querySelector("strong")!.textContent = categorias[dato.categoria] ?? dato.categoria;
    tarjeta.querySelector("p")!.textContent = mostrarDato(dato);
    tarjeta.querySelector(".source-label")!.textContent = dato.origen;
  }
  if (detalle) {
    const metadatos = document.createElement("dl");
    metadatos.className = "evidence-source";
    const fecha = dato.momento ? new Date(dato.momento) : null;
    const campos = [
      ["Registro", dato.tipo === "medicion" && medicion?.origen !== dato.origen && medicion?.origen ? dato.origen : ""],
      ["Autor", dato.autor],
      ["Fecha de registro", fecha && Number.isFinite(fecha.getTime()) ? fecha.toLocaleDateString("es-CL") : ""],
    ];
    for (const [etiqueta, valor] of campos) {
      if (!valor) continue;
      const termino = document.createElement("dt");
      termino.textContent = etiqueta;
      const descripcion = document.createElement("dd");
      descripcion.textContent = valor;
      metadatos.append(termino, descripcion);
    }
    if (metadatos.childElementCount) tarjeta.append(metadatos);
  }
  return tarjeta;
}

function botonOrigen(consultar: () => void): HTMLButtonElement {
  const boton = document.createElement("button");
  boton.type = "button";
  boton.className = "text-button";
  boton.innerHTML = `${iconoValoracion("ficha-clinica", 20)}<span>Consultar origen</span>`;
  boton.addEventListener("click", consultar);
  return boton;
}

export function montarEvidencias(
  panel: HTMLElement, intento: string, motivo: string, registros: Registros,
  volverAlChat: () => void, abrirHipotesis: () => void, abrirFicha: () => void, mostrarTurno: (id: string) => void,
): () => Promise<void> {
  panel.innerHTML = `
    <h2>Hallazgos obtenidos</h2>
    <div class="notebook-scroll stack tight">
      <article class="evidence-item" data-motivo-inicial>
        <p class="clinical-label">Motivo de consulta</p><h3></h3>
      </article>
      <div class="inset" data-sin-mediciones>
        <h3>Mediciones por explorar</h3>
        <p class="small-text">Los resultados aparecerán al completar una medición.</p>
      </div>
      <ul class="item-list" data-evidencias></ul>
    </div>
    <p class="small-text" role="status" data-carga hidden></p>
    <div class="button-row notebook-shortcuts">
      <button class="button quiet small" type="button" data-ir-chat>Volver al chat</button>
      <button class="button small" type="button" data-ir-hipotesis>Fundamentar hipótesis</button>
    </div>`;
  const inicial = panel.querySelector("[data-motivo-inicial]")!;
  inicial.querySelector("h3")!.textContent = motivo;
  inicial.append(botonOrigen(abrirFicha));
  panel.querySelector("[data-ir-chat]")!.addEventListener("click", volverAlChat);
  panel.querySelector("[data-ir-hipotesis]")!.addEventListener("click", abrirHipotesis);
  const lista = panel.querySelector<HTMLElement>("[data-evidencias]")!;
  const carga = panel.querySelector<HTMLElement>("[data-carga]")!;
  const sinMediciones = panel.querySelector<HTMLElement>("[data-sin-mediciones]")!;
  let solicitud = 0;

  function pintar(actual: Registros): void {
    const datos = actual.evidencias ?? [];
    const mediciones = new Map(actual.mediciones?.map((medicion) => [medicion.medicion_id, medicion]));
    lista.replaceChildren();
    lista.hidden = datos.length === 0;
    sinMediciones.hidden = datos.some((dato) => dato.tipo === "medicion");
    for (const dato of datos) {
      const medicion = dato.tipo === "medicion" ? mediciones.get(dato.origen_id) : undefined;
      const tarjeta = tarjetaEvidencia(dato, medicion);
      tarjeta.append(botonOrigen(() => {
        const detalle = tarjetaEvidencia(dato, medicion, true);
        if (dato.tipo === "turno") {
          const ir = document.createElement("button");
          ir.type = "button";
          ir.className = "text-button";
          ir.textContent = "Ir al mensaje";
          ir.addEventListener("click", () => {
            dialogo.close();
            mostrarTurno(dato.origen_id);
          });
          detalle.append(ir);
        }
        const dialogo = abrirDialogo("Origen de la evidencia", detalle);
      }));
      lista.append(tarjeta);
    }
  }

  pintar(registros);
  return async () => {
    const enCurso = ++solicitud;
    try {
      const actual = await api<Registros>("/intentos/" + encodeURIComponent(intento));
      if (enCurso !== solicitud || !panel.isConnected) return;
      pintar(actual);
      carga.hidden = true;
    } catch (error) {
      if (enCurso !== solicitud || !panel.isConnected) return;
      carga.textContent = error instanceof Error ? error.message : "No se pudieron actualizar las evidencias.";
      carga.hidden = false;
    }
  };
}
