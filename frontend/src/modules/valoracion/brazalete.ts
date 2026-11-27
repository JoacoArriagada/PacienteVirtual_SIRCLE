import { escenaBrazo, escenaColocada, iconoValoracion, retratoLR } from "./brazo.js";
import { prepararPresion } from "./presion.js";
import { metricasPresion } from "./historico-presion.js";

type Etapa = "inicio" | "preparado" | "colocado" | "midiendo" | "resultado";

export interface Lectura {
  medicion_id: string;
  valor: { sistolica: number; diastolica: number };
  unidad: string;
  fuente: string;
}

export class Brazalete {
  etapa: Etapa = "inicio";
  lectura: Lectura | null = null;
  readonly historico = new Map<string, Lectura>();

  preparar(): void {
    if (this.etapa === "inicio") this.etapa = "preparado";
  }

  colocar(): void {
    if (this.etapa === "preparado") this.etapa = "colocado";
  }

  async tomar(cargar: () => Promise<Lectura>): Promise<void> {
    if (this.etapa !== "colocado") return;
    this.etapa = "midiendo";
    try {
      this.lectura = await cargar();
      this.etapa = "resultado";
    } catch (error) {
      this.etapa = "colocado";
      throw error;
    }
  }

  registrar(): void {
    if (this.lectura) this.historico.set(this.lectura.medicion_id, this.lectura);
  }
}

const pasosPresion = [
  "Selecciona el brazalete disponible para el escenario.",
  "Coloca el brazalete y apoya el brazo.",
  "Confirma que el brazo permanece en posición.",
];

export function mostrarPreparacionBrazalete(main: HTMLElement, intento: string, volver: () => void): void {
  main.dataset.view = "procedure";
  main.innerHTML = `
    <section class="view procedure-view" aria-labelledby="page-title">
      <div class="page-heading with-back">
        <div class="heading-back">
          <button class="button" type="button" data-atencion>
            ${iconoValoracion("vista-volver", 20)}<span>Volver a la atención</span>
          </button>
        </div>
        <div class="heading-copy">
          <h1 id="page-title" tabindex="-1"></h1>
          <p class="muted">Brazalete y monitor · Simulación</p>
        </div>
      </div>
      <div class="procedure-grid" data-instrumento>
        <div class="panel">
          <div class="procedure-portrait">
            <img class="portrait" src="${retratoLR}" alt="">
            <strong>LR</strong><span class="muted">· +00 min</span>
          </div>
          <figure class="stack tight">
            <div class="device-art" id="device-art" data-phase="initial" data-positioned="false">
              <img class="procedure-photo" src="${escenaBrazo}" alt="LR en reposo, con el brazo apoyado" width="756" height="504">
              <img class="procedure-photo positioned-photo" src="${escenaColocada}" alt="" width="756" height="504">
            </div>
            <figcaption class="small-text muted">Brazalete y monitor · Escena de simulación</figcaption>
          </figure>
        </div>
        <section class="panel stack" aria-label="Controles del instrumento">
          <div class="procedure-state">
            <span>Estado del procedimiento</span><strong data-estado></strong>
          </div>
          <div class="stack" data-controles></div>
        </section>
      </div>
      <section class="panel stack" data-panel-historico aria-label="Histórico de presión" hidden></section>
    </section>`;
  const titulo = main.querySelector<HTMLElement>("#page-title")!;
  const escena = main.querySelector<HTMLElement>("#device-art")!;
  const estado = main.querySelector<HTMLElement>("[data-estado]")!;
  const controles = main.querySelector<HTMLElement>("[data-controles]")!;
  const instrumento = main.querySelector<HTMLElement>("[data-instrumento]")!;
  const panelHistorico = main.querySelector<HTMLElement>("[data-panel-historico]")!;
  const presion = prepararPresion(intento, { titulo, escena, estado, controles, instrumento, panelHistorico, inicial });
  main.querySelector("[data-atencion]")!.addEventListener("click", () => {
    if (presion.cerrar()) volver();
  });

  function inicial(): void {
    panelHistorico.hidden = true;
    instrumento.hidden = false;
    escena.dataset.phase = "initial";
    escena.dataset.positioned = "false";
    titulo.textContent = "Presión arterial · Inicial";
    estado.textContent = "Sin medir";
    controles.innerHTML = `
      <h2>Preparar el brazalete</h2>${metricasPresion()}
      <ol class="step-preview">${pasosPresion.map((paso) => `<li>${paso}</li>`).join("")}</ol>
      <p class="small-text muted">Duración de la simulación: 20 s.</p>
      <div class="button-row">
        <button type="button" class="button primary" data-preparar>
          ${iconoValoracion("instrumento-presion", 28)}<span>Preparar el brazalete</span>
        </button>
        <button type="button" class="button" data-historico><span>Histórico</span></button>
      </div>`;
    controles.querySelector("[data-preparar]")!.addEventListener("click", preparar);
    controles.querySelector("[data-historico]")!.addEventListener("click", presion.historico);
    titulo.focus();
  }

  function preparar(): void {
    escena.dataset.phase = "prepare";
    titulo.textContent = "Presión arterial · Preparación";
    estado.textContent = "Preparando";
    controles.innerHTML = `
      <div class="stack">
        <h2>Preparar el brazalete</h2>
        <fieldset>
          <legend>Confirma la preparación</legend>
          ${pasosPresion.map((paso, indice) => `
            <label class="check" for="step-${indice}">
              <input type="checkbox" id="step-${indice}" data-step="${indice}">
              <span>${paso}</span>
            </label>`).join("")}
        </fieldset>
        <div class="button-row">
          <button type="button" class="button primary" id="procedure-start" disabled>
            <svg class="icon" viewBox="0 0 24 24" aria-hidden="true" focusable="false"><path d="m8 5 12 7-12 7z"></path></svg>
            <span>Medir presión</span>
          </button>
          <button type="button" class="button" data-cancelar>
            ${iconoValoracion("cancelar-d939d1e1f0", 20)}<span>Cancelar preparación</span>
          </button>
        </div>
      </div>`;
    const casillas = Array.from(controles.querySelectorAll("input"));
    const medir = controles.querySelector<HTMLButtonElement>("#procedure-start")!;
    for (const casilla of casillas) {
      casilla.addEventListener("change", () => {
        escena.dataset.positioned = String(casillas[1].checked);
        medir.disabled = !casillas.every((paso) => paso.checked);
      });
    }
    medir.addEventListener("click", presion.comenzar);
    controles.querySelector("[data-cancelar]")!.addEventListener("click", inicial);
    casillas[0].focus();
  }

  inicial();
}

