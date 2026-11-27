import { abrirDialogo } from "../../app/dialogo.js";

export const escenaBrazo = "/recursos/simulacion/lr-presion.webp";
export const escenaColocada = "/recursos/simulacion/lr-presion-colocada.webp";
export const retratoLR = "/recursos/simulacion/retrato-de-lr-865a70387a.webp";

export function montarExtremidad(
  contenedor: HTMLElement, abrirPresion: () => void,
  instrumentos: readonly { nombre: string; etiqueta?: string; icono: string; exploracion?: boolean; abrir?: () => void }[],
): void {
  contenedor.innerHTML = `
    <h3>Signos vitales</h3>
    <div class="tool-grid">
      <button class="tool-button" type="button" title="Presión arterial" data-presion>
        ${iconoValoracion("instrumento-presion", 28)}<span>Presión</span>
      </button>
    </div>
    <h3>Métodos de examen</h3>
    <div class="tool-grid" data-exploraciones></div>
    <h3>Vista del paciente</h3>
    <div class="action-pair">
      <button class="button" type="button" data-brazo>${iconoValoracion("vista-brazo", 28)}<span>Ver brazo</span></button>
      <button class="button" type="button" data-ampliar>${iconoValoracion("vista-ampliar", 28)}<span>Ampliar</span></button>
    </div>`;
  contenedor.querySelector("[data-presion]")!.addEventListener("click", abrirPresion);
  const grupo = contenedor.querySelector(".tool-grid")!;
  const exploraciones = contenedor.querySelector("[data-exploraciones]")!;
  for (const instrumento of instrumentos) {
    const boton = document.createElement("button");
    boton.className = "tool-button";
    boton.type = "button";
    boton.innerHTML = iconoValoracion(instrumento.icono, 28) + "<span></span>";
    boton.querySelector("span")!.textContent = instrumento.etiqueta ?? instrumento.nombre;
    if (instrumento.abrir) {
      boton.title = instrumento.nombre;
      boton.addEventListener("click", instrumento.abrir);
    } else {
      boton.disabled = true;
      const estado = document.createElement("small");
      estado.textContent = "Sin datos";
      boton.append(estado);
    }
    (instrumento.exploracion ? exploraciones : grupo).append(boton);
  }
  contenedor.querySelector("[data-brazo]")!.addEventListener("click", () => abrirImagen(true));
  contenedor.querySelector("[data-ampliar]")!.addEventListener("click", () => abrirImagen(false));
}

export function iconoValoracion(archivo: string, tamano: number): string {
  return `<img class="asset-icon collection-icon" src="/recursos/simulacion/${archivo}.svg" alt="" width="${tamano}" height="${tamano}" aria-hidden="true">`;
}

function abrirImagen(brazo: boolean): void {
  const cuerpo = document.createElement("div");
  cuerpo.className = "image-dialog stack tight";
  cuerpo.innerHTML = '<figure><img><figcaption class="muted"></figcaption></figure>';
  const imagen = cuerpo.querySelector("img")!;
  imagen.src = brazo ? escenaBrazo : retratoLR;
  imagen.className = brazo ? "arm-image" : "portrait";
  imagen.alt = brazo ? "LR con el brazo apoyado" : "Retrato ampliado de LR";
  cuerpo.querySelector("figcaption")!.textContent = brazo ? "Brazo apoyado en la consulta" : "Preocupación";
  if (!brazo) {
    const reducir = document.createElement("button");
    reducir.type = "button";
    reducir.className = "button small";
    reducir.textContent = "Reducir imagen";
    reducir.addEventListener("click", () => dialogo.close());
    cuerpo.append(reducir);
  }
  const dialogo = abrirDialogo(brazo ? "Vista del brazo · LR" : "Paciente · LR", cuerpo);
}
