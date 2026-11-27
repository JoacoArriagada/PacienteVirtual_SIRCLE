export function abrirDialogo(titulo: string, contenido: Node): HTMLDialogElement {
  const dialogo = document.querySelector<HTMLDialogElement>("#dialog")!;
  dialogo.querySelector("#dialog-title")!.textContent = titulo;
  dialogo.querySelector("#dialog-body")!.replaceChildren(contenido);
  dialogo.showModal();
  return dialogo;
}
