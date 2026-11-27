import type { Instrumento } from "./instrumento.js";

export const respiracion = (ciclos: number): Instrumento => ({
  id: "respiracion", nombre: "Respiración", metodo: "Conteo por observación", icono: "respiracion-8e093006df",
  imagen: "/recursos/simulacion/lr-frente.webp",
  preparar: "Preparar el conteo", iniciar: "Contar respiraciones", duracion: 24,
  pasos: [
    "Confirma que el paciente está en reposo en la escena.",
    "Mantén el tórax a la vista.",
  ],
  instrucciones: "Observa la elevación y el descenso del tórax. Marca cada ciclo completo.",
  conteo: { ciclos, x: 35, y: 75 },
});
