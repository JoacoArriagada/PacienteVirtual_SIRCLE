export interface Sugerencia {
  id: string;
  texto: string;
}

export interface Pregunta {
  texto: string;
  sugerencia_id?: string;
}

export function montarSugerencias(
  contenedor: HTMLElement,
  sugerencias: Sugerencia[],
  enviar: (pregunta: Pregunta, clave: string) => Promise<void>,
): void {
  contenedor.innerHTML = `
    <details class="question-box" id="questions" ${sugerencias.length ? "" : "hidden"}>
      <summary>Preguntas de ejemplo</summary>
      <div class="question-list"></div>
    </details>
    <form class="chat-form" id="chat-form">
      <label class="small-text" for="question-input"><strong>Pregunta al paciente</strong></label>
      <div class="composer">
        <textarea id="question-input" rows="2" maxlength="1000" placeholder="Escribe una pregunta o elige un ejemplo"></textarea>
        <button class="button primary" type="submit" id="send-question" disabled><span>Enviar</span></button>
      </div>
      <p role="status" data-error-entrevista hidden></p>
    </form>`;
  const lista = contenedor.querySelector(".question-list")!;
  const formulario = contenedor.querySelector("form")!;
  const texto = formulario.querySelector("textarea")!;
  const boton = formulario.querySelector("button")!;
  const error = formulario.querySelector<HTMLElement>("[data-error-entrevista]")!;
  let seleccion: Sugerencia | undefined;
  let pendiente: { cuerpo: string; clave: string } | undefined;
  let enviando = false;

  function actualizar(): void {
    boton.disabled = enviando || !texto.value.trim();
    texto.readOnly = enviando;
    for (const opcion of lista.querySelectorAll<HTMLButtonElement>("button")) {
      opcion.disabled = enviando;
      opcion.setAttribute("aria-pressed", String(opcion.dataset.value === seleccion?.id));
    }
  }

  for (const sugerencia of sugerencias) {
    const opcion = document.createElement("button");
    opcion.className = "question";
    opcion.type = "button";
    opcion.dataset.value = sugerencia.id;
    opcion.textContent = sugerencia.texto;
    opcion.setAttribute("aria-pressed", "false");
    opcion.addEventListener("click", () => {
      seleccion = sugerencia;
      texto.value = sugerencia.texto;
      actualizar();
      texto.focus();
    });
    lista.append(opcion);
  }
  texto.addEventListener("input", () => {
    if (seleccion && texto.value !== seleccion.texto) seleccion = undefined;
    actualizar();
  });
  formulario.addEventListener("submit", async (evento) => {
    evento.preventDefault();
    const entrada = texto.value.trim();
    if (enviando || !entrada) return;
    const pregunta = seleccion
      ? { texto: entrada, sugerencia_id: seleccion.id }
      : { texto: entrada };
    const cuerpo = JSON.stringify(pregunta);
    if (pendiente?.cuerpo !== cuerpo) pendiente = { cuerpo, clave: crypto.randomUUID() };
    enviando = true;
    error.hidden = true;
    actualizar();
    try {
      await enviar(pregunta, pendiente.clave);
      pendiente = undefined;
      seleccion = undefined;
      texto.value = "";
    } catch (fallo) {
      error.textContent = fallo instanceof Error ? fallo.message : "Reintenta la pregunta.";
      error.hidden = false;
    } finally {
      enviando = false;
      actualizar();
      texto.focus();
    }
  });
}
