import type { MedicionPresion } from "./presion.js";

const series = [
  { clave: "sistolica", titulo: "Sistólica", color: "#2f65c4" },
  { clave: "diastolica", titulo: "Diastólica", color: "#9d435c" },
] as const;

export function metricasPresion(lectura?: MedicionPresion, sinLectura = "000"): string {
  return '<dl class="metrics">' + series.map(({ clave, titulo }) => `
    <div class="metric"><dt>${titulo}</dt><dd><strong>${lectura?.valor[clave] ?? sinLectura}</strong><span>mmHg</span></dd></div>`,
  ).join("") + "</dl>";
}

export function graficaPresion(lecturas: MedicionPresion[]): string {
  const ultimas = lecturas.slice(-10);
  const valores = ultimas.flatMap((toma) => series.map(({ clave }) => toma.valor[clave]));
  const menor = Math.min(...valores);
  const mayor = Math.max(...valores);
  const margen = Math.max(1, (mayor - menor) * 0.2);
  const inferior = menor - margen;
  const superior = mayor + margen;
  const x = (indice: number) => ultimas.length === 1 ? 157 : 47 + indice * 218 / (ultimas.length - 1);
  const y = (valor: number) => 119 - (valor - inferior) * 98 / (superior - inferior);
  const ticks = [inferior, (inferior + superior) / 2, superior];
  return `<div class="chart-grid pressure-chart"><figure class="chart">
    <h3>Presión arterial (mmHg)</h3>
    <svg viewBox="0 0 280 153" role="img" aria-label="Presión arterial por toma. Los valores exactos están en Tabla.">
      ${ticks.map((tick) => `<path d="M42 ${y(tick)}H270" stroke="#d5deed" fill="none"></path>
        <text x="36" y="${y(tick) + 4}" text-anchor="end">${Math.round(tick)}</text>`).join("")}
      ${series.map(({ clave, titulo, color }, indice) => `
        <polyline points="${ultimas.map((toma, i) => `${x(i)},${y(toma.valor[clave])}`).join(" ")}"
          fill="none" stroke="${color}" stroke-width="2.5" ${indice ? 'stroke-dasharray="5 4"' : ""}></polyline>
        ${ultimas.map((toma, i) => `<circle cx="${x(i)}" cy="${y(toma.valor[clave])}" r="4" fill="${color}">
          <title>${titulo} ${toma.valor[clave]} mmHg</title></circle>`).join("")}
      `).join("")}
      ${ultimas.map((_, indice) => (indice === 0 || indice === ultimas.length - 1 || ultimas.length <= 4)
        ? `<text x="${x(indice)}" y="141" text-anchor="middle">${lecturas.length - ultimas.length + indice + 1}</text>`
        : "").join("")}
    </svg>
    <figcaption class="chart-legend">${series.map(({ titulo, color }) =>
      `<span class="chart-key"><i style="--series:${color}"></i>${titulo}</span>`).join("")}</figcaption>
  </figure></div>`;
}
