import { textoMedicion, type Medicion } from "./instrumento.js";

const nombres: Record<string, string> = {
  presion: "Presión arterial",
  pulso: "Pulso",
  temperatura: "Temperatura",
  respiracion: "Frecuencia respiratoria",
  saturacion: "Saturación de oxígeno",
  peso: "Peso corporal",
};

type Lectura = Pick<Medicion, "instrumento" | "valor" | "unidad" | "origen"> & { momento?: string };

export function resumenMedicion(lectura: Lectura): HTMLElement {
  const fila = document.createElement("div");
  fila.className = "measurement-summary";
  fila.innerHTML = `
    <div><strong data-nombre-medicion></strong><p class="source-label" data-procedencia hidden></p></div>
    <div class="measurement-result"><strong data-lectura></strong></div>`;
  fila.querySelector("[data-nombre-medicion]")!.textContent = nombres[lectura.instrumento] ?? "Medición";
  const procedencia = fila.querySelector<HTMLElement>("[data-procedencia]")!;
  procedencia.textContent = lectura.origen ?? "";
  procedencia.hidden = !lectura.origen;
  const { ciclos, ...valor } = lectura.valor;
  fila.querySelector("[data-lectura]")!.textContent = textoMedicion({ ...lectura, valor });
  if (ciclos !== undefined) {
    const conteo = document.createElement("p");
    conteo.className = "source-label";
    conteo.textContent = ciclos === 1 ? "1 ciclo marcado" : `${ciclos} ciclos marcados`;
    procedencia.after(conteo);
  }

  if (lectura.momento) {
    const instante = new Date(lectura.momento);
    if (Number.isFinite(instante.getTime())) {
      const hora = document.createElement("time");
      hora.dateTime = lectura.momento;
      hora.className = "source-label";
      hora.textContent = instante.toLocaleTimeString("es-CL", {
        hour: "2-digit",
        minute: "2-digit",
        hourCycle: "h23",
      });
      fila.querySelector(".measurement-result")!.append(hora);
    }
  }
  return fila;
}
