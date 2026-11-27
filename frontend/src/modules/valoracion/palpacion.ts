import type { InstrumentoZonado } from "./exploracion.js";

export const palpacion: InstrumentoZonado = {
  id: "palpacion", nombre: "Palpación", metodo: "Exploración superficial", icono: "instrumento-palpacion",
  preparar: "Preparar la palpación", iniciar: "Palpar", duracion: 6,
  pasos: ["Explica el contacto y pide permiso en la simulación.", "Pregunta dónde siente molestia."],
  instrucciones: "La escena representa el contacto superficial en la zona elegida.",
  accesorio: { archivo: "/recursos/simulacion/palpacion.webp", clase: "palpation-hand" },
  zonas: [
    { id: "central", etiqueta: "Tórax anterior", imagen: "/recursos/simulacion/lr-frente.webp", x: 35, y: 75 },
    { id: "extremidad", etiqueta: "Pantorrilla derecha", imagen: "/recursos/simulacion/lr-pantorrilla.webp", x: 37, y: 39 },
  ],
};
