import { api, escribir, ErrorHttp } from "../../app/api.js";

type Momento = "pre" | "post";
interface Formulario {
  estado: "pendiente" | "respondido" | "omitido";
  items: {
    id: string;
    tipo: "percepcion" | "razonamiento" | "utilidad";
    enunciado: string;
    opciones: string[];
  }[];
}

export async function montarEncuesta(
  panel: HTMLElement, intento: string, momento: Momento, alResolver: () => Promise<void>,
): Promise<void> {
  const ruta = `/intentos/${encodeURIComponent(intento)}/encuestas/${momento}`;
  panel.classList.add("stack");
  panel.setAttribute("aria-busy", "true");
  try {
    const formulario = await api<Formulario>(ruta);
    if (!panel.isConnected) return;
    if (formulario.estado !== "pendiente") {
      await alResolver();
      return;
    }
    const form = document.createElement("form");
    form.className = "stack encuesta";
    const titulo = document.createElement("h3");
    titulo.textContent = momento === "pre" ? "Antes de la práctica" : "Después de entregar";
    const instruccion = document.createElement("p");
    instruccion.textContent = "La encuesta es voluntaria y sus respuestas se usan para evaluar la actividad educativa. Puedes dejar preguntas en blanco. Tu nota clínica es independiente.";
    form.append(titulo, instruccion);
    for (const item of formulario.items) {
      const grupo = document.createElement("fieldset");
      const enunciado = document.createElement("legend");
      enunciado.textContent = item.enunciado;
      grupo.append(enunciado);
      item.opciones.forEach((texto, indice) => {
        const etiqueta = document.createElement("label");
        const radio = document.createElement("input");
        radio.type = "radio";
        radio.name = item.id;
        radio.value = String(indice + 1);
        etiqueta.append(radio, item.tipo === "razonamiento" ? texto : `${indice + 1} · ${texto}`);
        grupo.append(etiqueta);
      });
      form.append(grupo);
    }
    const acciones = document.createElement("div");
    acciones.className = "button-row";
    acciones.innerHTML = `<button type="submit" class="button primary">Enviar respuestas</button>
      <button type="button" class="button" data-omitir>Omitir encuesta</button>`;
    const aviso = document.createElement("p");
    aviso.setAttribute("role", "status");
    form.append(acciones, aviso);
    panel.replaceChildren(form);

    let previa: { contenido: string; clave: string } | undefined;
    async function confirmar(estado: "respondido" | "omitido"): Promise<void> {
      const respuestas = estado === "omitido" ? {} :
        Object.fromEntries([...new FormData(form)].map(([item, valor]) => [item, Number(valor)]));
      if (estado === "respondido" && Object.keys(respuestas).length === 0) {
        aviso.textContent = "Elige al menos una respuesta u omite la encuesta.";
        return;
      }
      const datos = { estado, respuestas };
      const contenido = JSON.stringify(datos);
      const clave = previa?.contenido === contenido ? previa.clave : crypto.randomUUID();
      previa = { contenido, clave };
      const botones = [...acciones.querySelectorAll("button")];
      botones.forEach((boton) => { boton.disabled = true; });
      aviso.textContent = "";
      try {
        await api(ruta, escribir("PUT", datos, clave));
        if (panel.isConnected) await alResolver();
      } catch (error) {
        if (error instanceof ErrorHttp && error.estado === 409) {
          try {
            const actual = await api<Formulario>(ruta);
            if (actual.estado !== "pendiente" && panel.isConnected) {
              await alResolver();
              return;
            }
          } catch {
            // Se muestra el conflicto original y se permite volver a consultar.
          }
        }
        aviso.textContent = error instanceof Error ? error.message : "No se pudo resolver la encuesta.";
      } finally {
        botones.forEach((boton) => { boton.disabled = false; });
      }
    }
    form.addEventListener("submit", (evento) => {
      evento.preventDefault();
      void confirmar("respondido");
    });
    acciones.querySelector("[data-omitir]")!.addEventListener("click", () => { void confirmar("omitido"); });
  } catch (error) {
    const aviso = document.createElement("p");
    aviso.setAttribute("role", "status");
    aviso.textContent = error instanceof Error ? error.message : "No se pudo cargar la encuesta.";
    const reintentar = document.createElement("button");
    reintentar.className = "button";
    reintentar.textContent = "Volver a cargar la encuesta";
    reintentar.addEventListener("click", () => {
      reintentar.disabled = true;
      void montarEncuesta(panel, intento, momento, alResolver);
    });
    panel.replaceChildren(aviso, reintentar);
  } finally {
    panel.removeAttribute("aria-busy");
  }
}
