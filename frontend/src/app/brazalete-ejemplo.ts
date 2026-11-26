import { Brazalete, type Lectura } from "../modules/valoracion/brazalete.js";

function textoLectura(lectura: Lectura): string {
  return `${lectura.valor.sistolica}/${lectura.valor.diastolica} ${lectura.unidad}`;
}

export function montarBrazalete(raiz: HTMLElement): void {
  const brazalete = new Brazalete();
  const brazo = raiz.querySelector<HTMLImageElement>("#brazo")!;
  const estado = raiz.querySelector<HTMLElement>("#presion-estado")!;
  const historico = raiz.querySelector<HTMLUListElement>("#presion-historico")!;
  const botones = {
    preparar: raiz.querySelector<HTMLButtonElement>('[data-accion="preparar"]')!,
    colocar: raiz.querySelector<HTMLButtonElement>('[data-accion="colocar"]')!,
    tomar: raiz.querySelector<HTMLButtonElement>('[data-accion="tomar"]')!,
    registrar: raiz.querySelector<HTMLButtonElement>('[data-accion="registrar"]')!,
  };
  const mensajes: Record<Brazalete["etapa"], string> = {
    inicio: "Prepara el brazalete para iniciar.",
    preparado: "Brazalete preparado. Colócalo en el brazo.",
    colocado: "Brazalete colocado. Ya puedes tomar la presión.",
    midiendo: "Tomando la presión…",
    resultado: "Lectura disponible.",
  };

  function actualizar(): void {
    const { etapa, lectura } = brazalete;
    if (etapa === "colocado") {
      brazo.src = "/recursos/presion-colocada.webp";
      brazo.alt = "Brazo de LR con el brazalete colocado";
    }
    botones.preparar.disabled = etapa !== "inicio";
    botones.colocar.disabled = etapa !== "preparado";
    botones.tomar.disabled = etapa !== "colocado";
    botones.registrar.disabled = etapa !== "resultado";
    estado.textContent = lectura ? `${textoLectura(lectura)} · ${lectura.fuente}` : mensajes[etapa];
    historico.replaceChildren(...Array.from(brazalete.historico.values(), (fila) => {
      const elemento = document.createElement("li");
      elemento.dataset.medicionId = fila.medicion_id;
      elemento.textContent = textoLectura(fila);
      return elemento;
    }));
  }

  botones.preparar.addEventListener("click", () => {
    brazalete.preparar();
    actualizar();
  });
  botones.colocar.addEventListener("click", () => {
    brazalete.colocar();
    actualizar();
  });
  botones.tomar.addEventListener("click", async () => {
    const toma = brazalete.tomar(async () => {
      const respuesta = await fetch("/ejemplos/presion.json");
      if (!respuesta.ok) throw new Error("No se obtuvo la lectura de ejemplo.");
      return await respuesta.json() as Lectura;
    });
    actualizar();
    try {
      await toma;
      actualizar();
    } catch {
      actualizar();
      estado.textContent = "No se obtuvo la lectura de ejemplo. Vuelve a intentarlo.";
    }
  });
  botones.registrar.addEventListener("click", () => {
    brazalete.registrar();
    actualizar();
  });

  actualizar();
  raiz.hidden = false;
}
