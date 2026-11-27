import { montarEncuesta } from "./formulario.js";

export function mostrarPost(panel: HTMLElement, intento: string, continuar: () => Promise<void>): void {
  void montarEncuesta(panel, intento, "post", continuar);
}
