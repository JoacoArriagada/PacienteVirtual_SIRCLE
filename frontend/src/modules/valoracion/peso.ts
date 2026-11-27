import type { Instrumento } from "./instrumento.js";

export const peso: Instrumento = {
  id: "peso", nombre: "Peso", metodo: "Báscula digital", icono: "instrumento-peso",
  imagen: "/recursos/simulacion/lr-peso-listo.webp",
  preparar: "Preparar la báscula", iniciar: "Medir peso", duracion: 6,
  pasos: [
    "Comprueba que el indicador marca cero.",
    "Confirma el calzado retirado en la escena.",
    "Sitúa al paciente en el centro de la plataforma virtual.",
  ],
  instrucciones: "La lectura se está estabilizando.",
  escenaColocada: { archivo: "/recursos/simulacion/lr-peso.webp", paso: 2 },
};
