import type { Instrumento } from "./instrumento.js";

export const saturacion: Instrumento = {
  id: "saturacion", nombre: "Saturación", metodo: "Pulsioxímetro de dedo", icono: "instrumento-saturacion",
  imagen: "/recursos/semana5/lr-mano.webp",
  preparar: "Preparar el sensor", iniciar: "Medir saturación", duracion: 9,
  pasos: [
    "Apoya la mano del paciente.",
    "Coloca la pinza del sensor en el índice.",
    "Confirma que la mano se mantiene quieta.",
  ],
  instrucciones: "El sensor espera una señal estable.",
  sensor: { archivo: "/recursos/semana5/pulsioximetro.webp", clase: "pulse-clip", paso: 1, x: 87, y: 63 },
};
