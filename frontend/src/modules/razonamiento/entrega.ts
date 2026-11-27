import { api, ErrorHttp, escribir } from "../../app/api.js";
import type { VistaIntento } from "../intentos/ficha.js";
import { mostrarDevolucion } from "../devolucion/publicacion.js";
import { mostrarPost } from "../encuestas/post.js";
import { iconoValoracion, retratoLR } from "../valoracion/brazo.js";
import { mostrarDato } from "./apuntes.js";

export interface AcuseEntrega {
  entrega_id: string;
  revision: number;
  confirmada_en: string;
}

export function mostrarAcuse(contenedor: HTMLElement, acuse: AcuseEntrega, intento: VistaIntento): void {
  const panel = document.createElement("section");
  panel.className = "panel stack";
  panel.dataset.entregaId = acuse.entrega_id;
  const titulo = document.createElement("h2");
  titulo.textContent = "Entrega confirmada";
  const devolucion = document.createElement("div");
  devolucion.className = "stack tight";
  devolucion.setAttribute("aria-live", "polite");
  panel.append(titulo, devolucion);
  contenedor.prepend(panel);
  const continuar = () => mostrarDevolucion(devolucion, intento.intento_id, intento.evidencias ?? []);
  if (intento.post === "pendiente") mostrarPost(devolucion, intento.intento_id, continuar);
  else void continuar();
}

export function montarEntrega(
  acciones: HTMLElement, vista: HTMLElement, id: string, cambiosSinGuardar: () => boolean,
  terminar: (actual: VistaIntento) => void,
): void {
  const ruta = "/intentos/" + encodeURIComponent(id);
  const abrir = document.createElement("button");
  abrir.className = "text-button";
  abrir.type = "button";
  abrir.innerHTML = '<span aria-hidden="true">→</span> Revisar entrega';
  const aviso = document.createElement("p");
  aviso.className = "small-text";
  aviso.setAttribute("role", "status");
  acciones.append(abrir, aviso);

  abrir.addEventListener("click", async () => {
    abrir.disabled = true;
    aviso.textContent = "";
    try {
      const actual = await api<VistaIntento>(ruta);
      if (!acciones.isConnected) return;
      if (actual.entrega) {
        terminar(actual);
        return;
      }
      revisar(actual);
    } catch (error) {
      aviso.textContent = mensaje(error);
    } finally {
      abrir.disabled = false;
    }
  });

  function revisar(actual: VistaIntento): void {
    const revision = document.createElement("section");
    revision.className = "view";
    revision.setAttribute("aria-labelledby", "titulo-entrega");
    revision.innerHTML = `
      <div class="page-heading"><div class="heading-copy">
        <h1 id="titulo-entrega" tabindex="-1">Revisar antes de entregar</h1>
      </div></div>
      <div class="panel session-frame">
        <header class="session-bar">
          <img class="portrait" src="${retratoLR}" alt="">
          <h2 data-paciente></h2><span class="muted small-text" data-edad></span>
        </header>
        <div class="review-box stack relaxed">
        <h2>Revisa tu trabajo</h2>
        <div class="stack" data-registros></div>
        <p data-cambios hidden>Hay hipótesis sin guardar. Puedes volver y guardarlas antes de entregar.</p>
        <p><strong>Al entregar se cierra la edición de este intento.</strong></p>
        <div class="button-row between">
          <button class="button" type="button" data-editar>${iconoValoracion("vista-volver", 20)}<span>Seguir editando</span></button>
          <button class="button primary" type="button" data-confirmar>${iconoValoracion("confirmar-entrega", 20)}<span>Confirmar entrega</span></button>
        </div>
        <p role="status" data-aviso></p>
      </div></div>`;
    revision.querySelector<HTMLElement>("[data-cambios]")!.hidden = !cambiosSinGuardar();
    revision.querySelector("[data-paciente]")!.textContent = actual.introduccion?.paciente ?? "";
    revision.querySelector("[data-edad]")!.textContent = actual.introduccion ? `${actual.introduccion.edad} años` : "";
    const registros = revision.querySelector<HTMLElement>("[data-registros]")!;
    const turnos = actual.turnos ?? [];
    const evidencias = (actual.evidencias ?? []).filter((item) => item.tipo !== "orientacion");
    const apuntes = evidencias.filter((item) => item.tipo === "apunte").length;
    const hipotesis = actual.borrador?.hipotesis ?? [];
    apartado(registros, "Conversación", iconoValoracion("continuar-conversacion", 20), `${turnos.length} turnos`,
      turnos.map((turno) => turno.pregunta + " · " + turno.respuesta));
    apartado(registros, "Notas y evidencia", iconoValoracion("ficha-clinica", 28),
      `${apuntes} apuntes · ${evidencias.length - apuntes} registros`, evidencias.map(mostrarDato));
    const bombilla = '<svg class="icon" viewBox="0 0 24 24" aria-hidden="true" focusable="false"><path d="M9 18h6m-5 3h4m-5-6a6 6 0 1 1 6 0v3H9z"/></svg>';
    const resumen = hipotesis.length === 0 ? "Sin hipótesis guardada"
      : hipotesis.length === 1 ? "1 hipótesis guardada" : `${hipotesis.length} hipótesis guardadas`;
    apartado(registros, "Razonamiento", bombilla, resumen,
      hipotesis.map((item) => [item.titulo, item.explicacion].filter(Boolean).join(" · ")));
    vista.hidden = true;
    vista.after(revision);
    revision.querySelector<HTMLElement>("h1")!.focus({ preventScroll: true });
    window.scrollTo({ top: 0, behavior: "instant" });
    const editar = revision.querySelector<HTMLButtonElement>("[data-editar]")!;
    const confirmar = revision.querySelector<HTMLButtonElement>("[data-confirmar]")!;
    const estado = revision.querySelector<HTMLElement>("[data-aviso]")!;
    editar.addEventListener("click", () => {
      revision.remove();
      vista.hidden = false;
      abrir.focus();
    });
    const clave = crypto.randomUUID();
    confirmar.addEventListener("click", async () => {
      editar.disabled = true;
      confirmar.disabled = true;
      estado.textContent = "";
      try {
        const acuse = await api<AcuseEntrega>(ruta + "/entrega",
          escribir("POST", { revision_esperada: actual.revision }, clave));
        if (revision.isConnected) terminar({ ...actual, estado: "entregado", entrega: acuse });
      } catch (error) {
        estado.textContent = mensaje(error);
        if (!(error instanceof ErrorHttp && error.estado === 409)) confirmar.disabled = false;
      } finally {
        editar.disabled = false;
      }
    });
  }
}

function apartado(contenedor: HTMLElement, etiqueta: string, icono: string, resumen: string, elementos: string[]): void {
  const detalle = document.createElement("div");
  detalle.className = "review-item";
  detalle.innerHTML = `${icono}<div><h3></h3><p class="muted small-text"></p></div>
    <button class="button" type="button">${iconoValoracion("instrumento-inspeccion", 20)}<span>Revisar</span></button>`;
  detalle.querySelector("h3")!.textContent = etiqueta;
  detalle.querySelector("p")!.textContent = resumen;
  const boton = detalle.querySelector("button")!;
  boton.setAttribute("aria-label", "Revisar " + etiqueta);
  boton.addEventListener("click", () => {
    const dialogo = document.querySelector<HTMLDialogElement>("#dialog")!;
    dialogo.querySelector("#dialog-title")!.textContent = etiqueta;
    const lista = document.createElement("ul");
    for (const texto of elementos) {
      const fila = document.createElement("li");
      fila.textContent = texto;
      lista.append(fila);
    }
    const cuerpo = dialogo.querySelector("#dialog-body")!;
    if (elementos.length) cuerpo.replaceChildren(lista);
    else cuerpo.textContent = resumen;
    dialogo.showModal();
  });
  contenedor.append(detalle);
}

function mensaje(error: unknown): string {
  return error instanceof Error ? error.message : "No se pudo confirmar la entrega.";
}
