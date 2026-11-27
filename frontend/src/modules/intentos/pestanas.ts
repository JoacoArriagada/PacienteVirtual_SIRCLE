export function montarPestanas(barra: HTMLElement, alCambiar?: (id: string) => void) {
  const botones = Array.from(barra.querySelectorAll<HTMLButtonElement>('[role="tab"]'));
  const pestanas = botones.map((boton) => ({
    boton, panel: document.getElementById(boton.getAttribute("aria-controls")!)!,
  }));

  function activar(id: string, enfocar = false): void {
    let cambio = false;
    for (const { boton, panel } of pestanas) {
      const activa = panel.id === id;
      cambio ||= activa && panel.hidden;
      boton.setAttribute("aria-selected", String(activa));
      boton.tabIndex = activa ? 0 : -1;
      panel.hidden = !activa;
      if (activa && enfocar) boton.focus();
    }
    if (cambio) alCambiar?.(id);
  }

  for (const [indice, { boton, panel }] of pestanas.entries()) {
    boton.addEventListener("click", () => activar(panel.id));
    boton.addEventListener("keydown", (evento) => {
      const ultima = pestanas.length - 1;
      let siguiente: number;
      switch (evento.key) {
        case "ArrowRight": siguiente = indice === ultima ? 0 : indice + 1; break;
        case "ArrowLeft": siguiente = indice === 0 ? ultima : indice - 1; break;
        case "Home": siguiente = 0; break;
        case "End": siguiente = ultima; break;
        default: return;
      }
      evento.preventDefault();
      activar(pestanas[siguiente].panel.id, true);
    });
  }
  return activar;
}
