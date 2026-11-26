import type { InstrumentoZonado } from "./exploracion.js";

export const inspeccion: InstrumentoZonado = {
  id: "inspeccion", nombre: "Inspección", metodo: "Observación del tórax", icono: "instrumento-inspeccion",
  preparar: "Preparar la inspección", iniciar: "Inspeccionar", duracion: 6,
  pasos: ["Explica qué parte se va a observar.", "Confirma iluminación y zona visible."],
  instrucciones: "Observa el movimiento y el esfuerzo en la vista elegida.",
  zonas: [
    { id: "frontal", etiqueta: "Tórax anterior", imagen: "/recursos/semana5/lr-frente.webp", x: 35, y: 75 },
    { id: "posterior", etiqueta: "Tórax posterior", imagen: "/recursos/semana5/lr-espalda.webp", x: 50, y: 59 },
  ],
};
