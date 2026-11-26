import type { InstrumentoZonado } from "./exploracion.js";

export const auscultacion: InstrumentoZonado = {
  id: "auscultacion", nombre: "Auscultación", metodo: "Fonendoscopio", icono: "instrumento-auscultacion",
  preparar: "Preparar el fonendoscopio", iniciar: "Auscultar", duracion: 8,
  pasos: ["Explica el examen y confirma que la zona está visible.", "Prepara el diafragma del fonendoscopio."],
  instrucciones: "El diafragma se encuentra en el punto seleccionado.",
  accesorio: { archivo: "/recursos/semana5/fonendoscopio.webp", clase: "stethoscope" },
  modos: [
    { id: "posterior", etiqueta: "Espalda" },
    { id: "anterior", etiqueta: "Pecho" },
    { id: "cardiaco", etiqueta: "Corazón" },
  ],
  audio: true,
  zonas: [
    { id: "postLeft", etiqueta: "Base izquierda", modo: "posterior", imagen: "/recursos/semana5/lr-espalda.webp", x: 41, y: 70 },
    { id: "postRight", etiqueta: "Base derecha", modo: "posterior", imagen: "/recursos/semana5/lr-espalda.webp", x: 59, y: 70 },
    { id: "antRight", etiqueta: "Campo derecho", modo: "anterior", imagen: "/recursos/semana5/lr-frente.webp", x: 27, y: 68 },
    { id: "antLeft", etiqueta: "Campo izquierdo", modo: "anterior", imagen: "/recursos/semana5/lr-frente.webp", x: 44, y: 68 },
    { id: "aortic", etiqueta: "Foco aórtico", modo: "cardiaco", imagen: "/recursos/semana5/lr-frente.webp", x: 33, y: 69 },
    { id: "mitral", etiqueta: "Foco mitral", modo: "cardiaco", imagen: "/recursos/semana5/lr-frente.webp", x: 43, y: 81 },
  ],
};
