import type { Instrumento } from "./instrumento.js";

export const pulso: Instrumento = {
  id: "pulso", nombre: "Pulso", metodo: "Pulsioxímetro de dedo", icono: "signos-vitales-2aafb5e9fd",
  imagen: "/recursos/simulacion/lr-mano.webp",
  preparar: "Preparar el pulsioxímetro", iniciar: "Medir pulso", duracion: 24,
  pasos: [
    "Apoya la mano del paciente.",
    "Coloca el sensor en el índice.",
    "Confirma que la mano se mantiene quieta.",
  ],
  instrucciones: "Espera la lectura del pulso en el sensor.",
  sensor: { archivo: "/recursos/simulacion/pulsioximetro.webp", clase: "pulse-clip", paso: 1, x: 87, y: 63 },
};
