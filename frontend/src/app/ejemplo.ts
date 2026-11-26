import { montarBrazalete } from "./brazalete-ejemplo.js";

const estado = document.querySelector("[data-estado]");

if (estado) {
  estado.textContent = "Interfaz de ejemplo disponible.";
}

const valoracion = document.querySelector<HTMLElement>("#valoracion");
if (valoracion) {
  montarBrazalete(valoracion);
}
