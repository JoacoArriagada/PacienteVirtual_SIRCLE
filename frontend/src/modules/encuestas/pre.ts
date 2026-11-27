import { montarEncuesta } from "./formulario.js";
import { api } from "../../app/api.js";
import type { VistaIntento } from "../intentos/ficha.js";

export function mostrarPre(panel: HTMLElement, intento: string, continuar: (vista: VistaIntento) => void): void {
  void montarEncuesta(panel, intento, "pre", async () => {
    const actual = await api<VistaIntento>(`/intentos/${encodeURIComponent(intento)}`);
    if (panel.isConnected) continuar(actual);
  });
}
