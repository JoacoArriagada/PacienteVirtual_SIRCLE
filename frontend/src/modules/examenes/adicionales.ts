import { montarEcg, type Orden } from "./ecg.js";

const examenes = [
  { nombre: "Radiografía de tórax", categoria: "Imágenes", palabras: "rx placa", disponible: false },
  { nombre: "Hemograma", categoria: "Laboratorio", palabras: "sangre", disponible: false },
  { nombre: "Electrocardiograma inicial", categoria: "Funcional", palabras: "ecg ekg", disponible: true },
];

function normalizar(texto: string): string {
  return texto.normalize("NFD").replace(/\p{Diacritic}/gu, "").toLowerCase().trim();
}

export function montarAdicionales(panel: HTMLElement, intento: string, ordenes: Orden[], alConsultar: () => void): void {
  panel.innerHTML = `
    <div class="stack tight exam-filters">
      <div class="stack tight">
        <label for="exam-search">Buscar examen</label>
        <input id="exam-search" type="search" placeholder="Nombre o categoría">
      </div>
      <div class="stack tight">
        <label for="exam-category">Categoría</label>
        <select id="exam-category"><option value="">Todas</option></select>
      </div>
    </div>
    <div class="exam-results stack tight" data-exam-results></div>
    <p class="empty small-text" role="status" data-sin-examenes hidden>Sin coincidencias. Prueba con otro nombre o categoría.</p>`;
  const busqueda = panel.querySelector("input")!;
  const categoria = panel.querySelector("select")!;
  const resultados = panel.querySelector("[data-exam-results]")!;
  const vacio = panel.querySelector<HTMLElement>("[data-sin-examenes]")!;
  for (const grupo of new Set(examenes.map((examen) => examen.categoria))) {
    categoria.add(new Option(grupo, grupo));
  }
  const tarjetas = examenes.map((examen) => {
    const tarjeta = document.createElement("article");
    tarjeta.className = "tile stack tight";
    if (examen.disponible) {
      montarEcg(tarjeta, intento, ordenes.find((orden) => orden.examen === "ecg-inicial"), alConsultar);
    } else {
      tarjeta.innerHTML = '<h4></h4><p class="small-text muted"></p>';
      tarjeta.querySelector("h4")!.textContent = examen.nombre;
      tarjeta.querySelector("p")!.textContent = `${examen.categoria} · No disponible para LR`;
    }
    resultados.append(tarjeta);
    return { tarjeta, categoria: examen.categoria, texto: normalizar(`${examen.nombre} ${examen.categoria} ${examen.palabras}`) };
  });

  function filtrar(): void {
    const consulta = normalizar(busqueda.value);
    for (const item of tarjetas) {
      item.tarjeta.hidden = !item.texto.includes(consulta) || Boolean(categoria.value && categoria.value !== item.categoria);
    }
    vacio.hidden = tarjetas.some(({ tarjeta }) => !tarjeta.hidden);
    resultados.scrollTop = 0;
  }
  busqueda.addEventListener("input", filtrar);
  categoria.addEventListener("change", filtrar);
}
