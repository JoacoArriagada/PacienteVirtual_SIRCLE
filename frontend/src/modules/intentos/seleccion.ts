import { api, ErrorHttp, escribir } from "../../app/api.js";
import { mostrarSupervision } from "../devolucion/revision.js";
import { mostrarFicha, type VistaIntento } from "./ficha.js";

interface Actividad {
  actividad_id: string;
  nombre: string;
  tipo: string;
  pre: string;
  intento_id: string | null;
  estado?: string;
}

const main = document.querySelector<HTMLElement>("#main")!;
const guardarEstado = document.querySelector<HTMLElement>("#storage-status")!;

export async function iniciarSeleccion(): Promise<void> {
  const dialogo = document.querySelector<HTMLDialogElement>("#dialog")!;
  dialogo.querySelector("[data-do=close]")?.addEventListener("click", () => dialogo.close());
  document.querySelector('[data-do="roles"]')?.addEventListener("click", vistaAcceso);
  document.querySelector('[data-do="exit"]')?.addEventListener("click", async () => {
    try {
      await api<void>("/sesiones", { method: "DELETE" });
      vistaAcceso();
    } catch (error) {
      aviso(error);
    }
  });
  document.querySelector('[data-do="home"]')?.addEventListener("click", (evento) => {
    evento.preventDefault();
    void cargarCatalogo();
  });
  document.querySelector('[data-do="toggle-nav"]')?.addEventListener("click", (evento) => {
    const boton = evento.currentTarget as HTMLButtonElement;
    const abierto = boton.getAttribute("aria-expanded") !== "true";
    boton.setAttribute("aria-expanded", String(abierto));
    document.querySelector<HTMLElement>("#module-nav")!.dataset.expanded = String(abierto);
  });
  const intento = location.hash.match(/^#intento\/([0-9a-z-]+)$/i)?.[1];
  if (intento) {
    try {
      abrirVista(await api<VistaIntento>("/intentos/" + encodeURIComponent(intento)));
      return;
    } catch (error) {
      if (!(error instanceof ErrorHttp && [401, 403].includes(error.estado))) {
        vistaAcceso();
        aviso(error);
        return;
      }
    }
  }
  await cargarCatalogo();
}

async function cargarCatalogo(): Promise<void> {
  try {
    const sesion = await api<{ rol: string }>("/sesiones");
    document.querySelector<HTMLElement>("#role-label")!.textContent = sesion.rol === "docente" ? "Docente" : "Estudiante";
    if (sesion.rol === "docente") {
      await mostrarSupervision(main);
      return;
    }
    const actividades = await api<Actividad[]>("/actividades");
    history.replaceState(null, "", "#pacientes");
    vistaCatalogo(actividades);
  } catch (error) {
    vistaAcceso();
    if (!(error instanceof ErrorHttp && error.estado === 401)) aviso(error);
  }
}

function vistaAcceso(): void {
  history.replaceState(null, "", "#pacientes");
  guardarEstado.textContent = "";
  main.dataset.view = "acceso";
  main.innerHTML = `
    <section class="view" aria-labelledby="page-title">
      <div class="page-heading"><div class="heading-copy">
        <h1 id="page-title" tabindex="-1">Entrar a SIRCLE</h1>
      </div></div>
      <form class="panel stack">
        <label class="field" for="identidad">Identidad ficticia</label>
        <select id="identidad" name="identidad">
          <option value="estudiante-lr">Estudiante de LR</option>
          <option value="estudiante-sin-asignacion">Estudiante sin asignación</option>
          <option value="docente-demo">Docente de ensayo</option>
        </select>
        <div class="button-row"><button type="submit" class="button primary">Entrar</button></div>
        <p data-aviso role="status"></p>
      </form>
    </section>`;
  const formulario = main.querySelector("form")!;
  formulario.addEventListener("submit", async (evento) => {
    evento.preventDefault();
    const boton = formulario.querySelector("button")!;
    boton.disabled = true;
    try {
      const identidad = new FormData(formulario).get("identidad");
      const sesion = await api<{ rol: string }>("/sesiones", escribir("POST", { identidad }));
      document.querySelector<HTMLElement>("#role-label")!.textContent = sesion.rol === "docente" ? "Docente" : "Estudiante";
      await cargarCatalogo();
    } catch (error) {
      aviso(error);
    } finally {
      boton.disabled = false;
    }
  });
  enfocarTitulo();
}

function vistaCatalogo(actividades: Actividad[]): void {
  main.dataset.view = "catalog";
  const plantilla = document.querySelector<HTMLTemplateElement>("#plantilla-catalogo")!;
  main.replaceChildren(plantilla.content.cloneNode(true));
  const raiz = main.querySelector(".catalog")!;
  const contenedor = raiz.querySelector("#catalog-cards")!;
  const contador = raiz.querySelector("#catalog-count")!;
  const buscar = raiz.querySelector<HTMLInputElement>("#patient-search")!;
  const pestanas = [...raiz.querySelectorAll<HTMLButtonElement>('[data-do="catalog-state"]')];
  const filtro = raiz.querySelector<HTMLButtonElement>('[data-do="catalog-filter"]')!;
  const dialogo = document.querySelector<HTMLDialogElement>("#dialog")!;
  const error = document.createElement("p");
  error.setAttribute("role", "status");
  error.dataset.aviso = "";
  raiz.append(error);
  let vista = "all";
  guardarEstado.textContent = "Sesión local";

  function pintar(): void {
    const texto = buscar.value.trim().toLocaleLowerCase("es-CL");
    const visibles = actividades.filter((actividad) =>
      actividad.nombre.toLocaleLowerCase("es-CL").includes(texto)
      && (vista === "all" || (vista === "paused" && actividad.intento_id && actividad.estado !== "entregado")
        || (vista === "done" && actividad.estado === "entregado")));
    contenedor.replaceChildren(...visibles.map(tarjeta));
    contador.textContent = visibles.length === 1 ? "1 paciente" : `${visibles.length} pacientes`;
    for (const boton of pestanas) boton.setAttribute("aria-pressed", String(boton.dataset.value === vista));
  }

  function tarjeta(actividad: Actividad): DocumentFragment {
    const datos = document.querySelector<HTMLTemplateElement>("#plantilla-lr")!;
    const tarjeta = datos.content.cloneNode(true) as DocumentFragment;
    const accion = tarjeta.querySelector<HTMLButtonElement>(".card-cta .primary")!;
    const estado = tarjeta.querySelector<HTMLElement>(".state")!;
    if (actividad.intento_id) {
      const entregada = actividad.estado === "entregado";
      estado.textContent = entregada ? "Entregado" : "En curso";
      estado.dataset.tone = entregada ? "success" : "warning";
      accion.querySelector("span")!.textContent = entregada ? "Ver entrega" : "Continuar";
      accion.dataset.do = "attempt-open";
      accion.dataset.value = actividad.intento_id;
    }
    tarjeta.querySelector<HTMLButtonElement>('[data-do="patient-info"]')!.addEventListener("click", () => {
      mostrarDialogo(dialogo, "Información de LR", actividad.nombre + " · " + (estado.textContent ?? ""));
    });
    accion.addEventListener("click", async () => {
      accion.disabled = true;
      try {
        const intento = actividad.intento_id
          ? await api<VistaIntento>("/intentos/" + encodeURIComponent(actividad.intento_id))
          : await api<VistaIntento>("/intentos", escribir("POST", { actividad_id: actividad.actividad_id }, crypto.randomUUID()));
        abrirVista(intento);
      } catch (error) {
        aviso(error);
      } finally {
        accion.disabled = false;
      }
    });
    return tarjeta;
  }

  filtro.addEventListener("click", () => {
    mostrarDialogo(dialogo, "Filtrar pacientes", "Busca por nombre o elige el estado de los casos.");
  });
  for (const boton of pestanas) boton.addEventListener("click", () => {
    vista = boton.dataset.value ?? "all";
    pintar();
  });
  buscar.addEventListener("input", pintar);
  pintar();
  enfocarTitulo();
}

function mostrarDialogo(dialogo: HTMLDialogElement, titulo: string, texto: string): void {
  dialogo.querySelector("#dialog-title")!.textContent = titulo;
  dialogo.querySelector("#dialog-body")!.textContent = texto;
  dialogo.showModal();
}

function abrirVista(intento: VistaIntento): void {
  history.replaceState(null, "", "#intento/" + encodeURIComponent(intento.intento_id));
  mostrarFicha(main, intento, () => { void cargarCatalogo(); });
}

function aviso(error: unknown): void {
  const destino = main.querySelector("[data-aviso]") ?? guardarEstado;
  destino.textContent = error instanceof Error ? error.message : "No se pudo completar la solicitud.";
}

function enfocarTitulo(): void {
  main.querySelector<HTMLElement>("#page-title")?.focus();
}
