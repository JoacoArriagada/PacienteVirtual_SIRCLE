import { api, escribir } from "../../app/api.js";

export interface Orden {
  orden_id: string;
  examen: string;
}

interface Informe {
  informe_id: string;
  resultado: string;
  origen: string;
}

const ECG_INICIAL = "ecg-inicial";

export function montarEcg(panel: HTMLElement, intento: string, orden?: Orden): void {
  const ruta = "/intentos/" + encodeURIComponent(intento) + "/ordenes";
  panel.className = "panel stack";
  panel.setAttribute("aria-labelledby", "titulo-examenes");
  panel.innerHTML = `
    <h2 id="titulo-examenes">Exámenes</h2>
    <h3>Electrocardiograma inicial</h3>
    <p role="status" data-estado></p>
    <div class="button-row">
      <button type="button" class="button primary" data-solicitar>Solicitar ECG inicial</button>
      <button type="button" class="button" data-consultar>Consultar informe</button>
    </div>
    <div class="stack tight" data-confirmacion hidden>
      <p>¿Confirmas la orden del ECG inicial para este intento?</p>
      <div class="button-row">
        <button type="button" class="button primary" data-confirmar>Confirmar orden</button>
        <button type="button" class="button" data-cancelar>Cancelar orden</button>
      </div>
    </div>
    <div class="stack tight" data-informe hidden>
      <h4>Informe del ECG inicial</h4>
      <p data-resultado></p>
      <p class="muted small-text" data-origen></p>
    </div>`;
  const elemento = (selector: string) => panel.querySelector<HTMLElement>(selector)!;
  const boton = (selector: string) => panel.querySelector<HTMLButtonElement>(selector)!;
  const estado = elemento("[data-estado]");
  const confirmacion = elemento("[data-confirmacion]");
  const solicitar = boton("[data-solicitar]");
  const confirmar = boton("[data-confirmar]");
  const cancelar = boton("[data-cancelar]");
  const consultar = boton("[data-consultar]");
  const resultado = elemento("[data-informe]");
  let clave: string | undefined;
  actualizar();

  function actualizar(): void {
    estado.textContent = orden ? "Orden confirmada." : "Informe reservado hasta confirmar la orden.";
    solicitar.disabled = Boolean(orden) || !confirmacion.hidden;
  }

  solicitar.addEventListener("click", () => {
    confirmacion.hidden = false;
    actualizar();
    confirmar.focus();
  });
  cancelar.addEventListener("click", () => {
    confirmacion.hidden = true;
    actualizar();
    solicitar.focus();
  });
  confirmar.addEventListener("click", async () => {
    confirmar.disabled = true;
    cancelar.disabled = true;
    consultar.disabled = true;
    clave ??= crypto.randomUUID();
    try {
      orden = await api<Orden>(ruta, escribir("POST", { examen: ECG_INICIAL, accion: "confirmar" }, clave));
      confirmacion.hidden = true;
      actualizar();
    } catch (error) {
      estado.textContent = error instanceof Error ? error.message : "No se pudo confirmar la orden.";
    } finally {
      confirmar.disabled = false;
      cancelar.disabled = false;
      consultar.disabled = false;
    }
    if (orden) consultar.focus();
  });
  consultar.addEventListener("click", async () => {
    consultar.disabled = true;
    try {
      const informe = await api<Informe>(ruta + "/" + encodeURIComponent(orden?.orden_id ?? ECG_INICIAL));
      elemento("[data-resultado]").textContent = informe.resultado;
      elemento("[data-origen]").textContent = informe.origen;
      resultado.dataset.informeId = informe.informe_id;
      resultado.hidden = false;
      estado.textContent = "Informe disponible.";
    } catch (error) {
      estado.textContent = error instanceof Error ? error.message : "No se pudo consultar el informe.";
    } finally {
      consultar.disabled = false;
    }
  });
}
