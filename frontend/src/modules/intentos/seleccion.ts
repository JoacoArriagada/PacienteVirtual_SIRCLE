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

interface Disponibilidad {
  base: { disponible: boolean };
  modelo: { disponible: boolean; aviso?: string };
}

const main = document.querySelector<HTMLElement>("#main")!;
const guardarEstado = document.querySelector<HTMLElement>("#storage-status")!;
const rolActual = document.querySelector<HTMLElement>("#role-label")!;
const cambiarRol = document.querySelector<HTMLButtonElement>("#role-button")!;
let vistaActiva = 0;
let cambiandoSesion = false;

function iniciarVista(): () => boolean {
  const actual = ++vistaActiva;
  return () => actual === vistaActiva;
}

function mostrarRol(rol: string): void {
  rolActual.textContent = rol === "docente" ? "Docente" : "Estudiante";
  cambiarRol.setAttribute("aria-label", "Cambiar rol. Rol actual: " + rolActual.textContent);
}

export async function iniciarSeleccion(): Promise<void> {
  const dialogo = document.querySelector<HTMLDialogElement>("#dialog")!;
  dialogo.querySelector("[data-do=close]")?.addEventListener("click", () => dialogo.close());
  document.querySelector('[data-do="service-status"]')?.addEventListener("click", () => mostrarDisponibilidad(dialogo));
  cambiarRol.addEventListener("click", () => { if (!cambiandoSesion) vistaAcceso(); });
  const salir = document.querySelector<HTMLButtonElement>('[data-do="exit"]')!;
  salir.addEventListener("click", async () => {
    if (cambiandoSesion) return;
    cambiandoSesion = true;
    vistaAcceso();
    const entrar = main.querySelector<HTMLButtonElement>('button[type="submit"]')!;
    entrar.disabled = true;
    try {
      await api<void>("/sesiones", { method: "DELETE" });
    } catch (error) {
      aviso(error);
    } finally {
      cambiandoSesion = false;
      entrar.disabled = false;
    }
  });
  document.querySelector('[data-do="home"]')?.addEventListener("click", (evento) => {
    evento.preventDefault();
    if (!cambiandoSesion) void cargarCatalogo();
  });
  document.querySelector('[data-do="toggle-nav"]')?.addEventListener("click", (evento) => {
    const boton = evento.currentTarget as HTMLButtonElement;
    const abierto = boton.getAttribute("aria-expanded") !== "true";
    boton.setAttribute("aria-expanded", String(abierto));
    document.querySelector<HTMLElement>("#module-nav")!.dataset.expanded = String(abierto);
  });
  const intento = location.hash.match(/^#intento\/([0-9a-z-]+)$/i)?.[1];
  if (intento) {
    const vigente = iniciarVista();
    try {
      const actual = await api<VistaIntento>("/intentos/" + encodeURIComponent(intento));
      if (vigente()) abrirVista(actual, vigente);
      return;
    } catch (error) {
      if (!vigente()) return;
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
  const vigente = iniciarVista();
  try {
    const sesion = await api<{ rol: string }>("/sesiones");
    if (!vigente()) return;
    mostrarRol(sesion.rol);
    if (sesion.rol === "docente") {
      await mostrarSupervision(main, vigente);
      return;
    }
    const actividades = await api<Actividad[]>("/actividades");
    if (!vigente()) return;
    history.replaceState(null, "", "#pacientes");
    vistaCatalogo(actividades);
  } catch (error) {
    if (!vigente()) return;
    vistaAcceso();
    if (!(error instanceof ErrorHttp && error.estado === 401)) aviso(error);
  }
}

function vistaAcceso(): void {
  iniciarVista();
  mostrarRol("estudiante");
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
    if (cambiandoSesion) return;
    cambiandoSesion = true;
    iniciarVista();
    const boton = formulario.querySelector("button")!;
    boton.disabled = true;
    try {
      const identidad = new FormData(formulario).get("identidad");
      await api("/sesiones", escribir("POST", { identidad }));
    } catch (error) {
      aviso(error);
      return;
    } finally {
      cambiandoSesion = false;
      boton.disabled = false;
    }
    await cargarCatalogo();
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
  const altas = new Map<string, string>();
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
    const titulo = tarjeta.querySelector("h2")!;
    titulo.id = "card-" + actividad.actividad_id;
    tarjeta.querySelector("article")!.setAttribute("aria-labelledby", titulo.id);
    if (actividades.length > 1) titulo.textContent = actividad.nombre;
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
      if (cambiandoSesion) return;
      const vigente = iniciarVista();
      accion.disabled = true;
      const clave = altas.get(actividad.actividad_id) ?? crypto.randomUUID();
      altas.set(actividad.actividad_id, clave);
      try {
        const intento = actividad.intento_id
          ? await api<VistaIntento>("/intentos/" + encodeURIComponent(actividad.intento_id))
          : await api<VistaIntento>("/intentos", escribir("POST", { actividad_id: actividad.actividad_id }, clave));
        if (vigente()) abrirVista(intento, vigente);
      } catch (error) {
        if (vigente()) aviso(error);
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

function mostrarDisponibilidad(dialogo: HTMLDialogElement): void {
  mostrarDialogo(dialogo, "Estado de la práctica", "");
  const informe = document.createElement("div");
  informe.setAttribute("aria-live", "polite");
  const actualizar = document.createElement("button");
  actualizar.type = "button";
  actualizar.className = "button";
  actualizar.textContent = "Actualizar estado";
  dialogo.querySelector("#dialog-body")!.replaceChildren(informe, actualizar);
  actualizar.addEventListener("click", () => { void consultar(); });
  void consultar();

  async function consultar(): Promise<void> {
    actualizar.disabled = true;
    informe.textContent = "Consultando disponibilidad…";
    try {
      const estado = await api<Disponibilidad>("/estado");
      if (!informe.isConnected || !dialogo.open) return;
      const practica = estado.base.disponible;
      const inferencia = practica && estado.modelo.disponible;
      const lista = document.createElement("ul");
      for (const texto of [
        "Práctica · " + (practica ? "Disponible" : "No disponible"),
        "Diálogo libre · " + (inferencia ? "Disponible" : "No disponible"),
        "Evaluación · " + (inferencia ? "Disponible" : practica ? "En espera de inferencia" : "No disponible"),
      ]) {
        const linea = document.createElement("li");
        linea.textContent = texto;
        lista.append(linea);
      }
      informe.replaceChildren(lista);
      if (estado.modelo.aviso && !estado.modelo.disponible) {
        const aviso = document.createElement("p");
        aviso.textContent = estado.modelo.aviso;
        informe.append(aviso);
      }
    } catch (error) {
      if (informe.isConnected) informe.textContent = error instanceof Error ? error.message : "No se pudo consultar el estado.";
    } finally {
      actualizar.disabled = false;
    }
  }
}

function abrirVista(intento: VistaIntento, vigente: () => boolean): void {
  history.replaceState(null, "", "#intento/" + encodeURIComponent(intento.intento_id));
  mostrarFicha(main, intento, () => { if (vigente()) void cargarCatalogo(); }, vigente);
}

function aviso(error: unknown): void {
  const destino = main.querySelector("[data-aviso]") ?? guardarEstado;
  destino.textContent = error instanceof Error ? error.message : "No se pudo completar la solicitud.";
}

function enfocarTitulo(): void {
  main.querySelector<HTMLElement>("#page-title")?.focus();
}
