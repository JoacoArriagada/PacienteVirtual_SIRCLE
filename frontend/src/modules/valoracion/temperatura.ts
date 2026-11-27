import type { Instrumento } from "./instrumento.js";

export const temperatura: Instrumento = {
  id: "temperatura", nombre: "Temperatura", metodo: "Termómetro frontal por infrarrojos", icono: "instrumento-temperatura",
  imagen: "/recursos/simulacion/lr-frente.webp",
  preparar: "Preparar el termómetro", iniciar: "Medir temperatura", duracion: 4,
  pasos: [
    "Deja la frente descubierta.",
    "Alinea el termómetro con la frente.",
    "Confirma la distancia indicada en la demostración.",
  ],
  instrucciones: "El termómetro está tomando la lectura frontal.",
  sensor: { archivo: "/recursos/simulacion/termometro.webp", clase: "thermometer", paso: "preparacion", x: 46, y: 15 },
};
