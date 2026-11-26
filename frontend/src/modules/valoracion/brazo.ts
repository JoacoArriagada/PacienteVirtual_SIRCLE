export const escenaBrazo = "/recursos/semana5/lr-presion.webp";
export const escenaColocada = "/recursos/semana5/lr-presion-colocada.webp";
export const retratoLR = "/recursos/semana5/retrato-de-lr-865a70387a.webp";

export function montarExtremidad(
  contenedor: HTMLElement, abrirPresion: () => void,
  instrumentos: readonly { nombre: string; icono: string; exploracion?: boolean; abrir: () => void }[],
): void {
  contenedor.innerHTML = `
    <h2>Acciones y exámenes</h2>
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
  for (const instrumento of instrumentos) {
    const boton = document.createElement("button");
    boton.className = "tool-button";
    boton.type = "button";
    boton.innerHTML = iconoValoracion(instrumento.icono, 28) + "<span></span>";
    boton.querySelector("span")!.textContent = instrumento.nombre;
    boton.addEventListener("click", instrumento.abrir);
    (instrumento.exploracion ? contenedor.querySelector("[data-exploraciones]")! : grupo).append(boton);
  }
  contenedor.querySelector("[data-brazo]")!.addEventListener("click", () => abrirImagen(true));
  contenedor.querySelector("[data-ampliar]")!.addEventListener("click", () => abrirImagen(false));
}

export function iconoValoracion(archivo: string, tamano: number): string {
  return `<img class="asset-icon collection-icon" src="/recursos/semana5/${archivo}.svg" alt="" width="${tamano}" height="${tamano}" aria-hidden="true">`;
}

function abrirImagen(brazo: boolean): void {
  const dialogo = document.querySelector<HTMLDialogElement>("#dialog")!;
  dialogo.querySelector("#dialog-title")!.textContent = brazo ? "Vista del brazo · LR" : "Paciente · LR";
  const cuerpo = dialogo.querySelector("#dialog-body")!;
  cuerpo.innerHTML = '<div class="image-dialog"><figure><img><figcaption class="muted"></figcaption></figure></div>';
  const imagen = cuerpo.querySelector<HTMLImageElement>("img")!;
  imagen.src = brazo ? escenaBrazo : retratoLR;
  imagen.className = brazo ? "arm-image" : "portrait";
  imagen.alt = brazo ? "LR con el brazo apoyado" : "Retrato ampliado de LR";
  cuerpo.querySelector("figcaption")!.textContent = brazo ? "Brazo apoyado en la consulta" : "Preocupación";
  dialogo.showModal();
}
