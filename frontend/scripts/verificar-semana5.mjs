import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const original = new URL("../referencia/semana5/Paciente_Virtual.html", import.meta.url);
const html = await readFile(original, "utf8");
const css = await readFile(new URL("../src/app/semana5.css", import.meta.url), "utf8");
assert.equal(css, html.match(/<style>([\s\S]*?)<\/style>/)?.[1]);
const comparacion = process.argv[2];
if (comparacion) await mkdir(comparacion, { recursive: true });
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
const selectores = [
  ".masthead", ".head-button.account-role", "#module-nav", "main",
  ".view.catalog", "#page-title", ".heading-actions .button", ".tab",
  "#patient-search", ".patient-card", ".patient-card .portrait",
  ".patient-name-line h2", ".patient-card .state", ".card-cta .button", ".app-footer",
];

try {
  const pruebas = [];
  for (const [nombre, direccion] of [
    ["original", original.href],
    ["extraccion", "http://127.0.0.1:4173/semana5"],
  ]) {
    const pagina = await navegador.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
    await pagina.goto(direccion);
    await pagina.locator(".patient-card").last().waitFor();
    await pagina.evaluate(async () => {
      await document.fonts.ready;
      await Promise.all([...document.images].map((imagen) => imagen.decode()));
    });
    const controles = await pagina.evaluate((selectores) => {
      return selectores.map((selector) => {
        const elemento = document.querySelector(selector);
        if (!elemento) throw new Error("Falta " + selector);
        const ventana = elemento.getBoundingClientRect();
        const estilo = getComputedStyle(elemento);
        return {
          selector, caja: { x: ventana.x, y: ventana.y, ancho: ventana.width, alto: ventana.height },
          clases: elemento.getAttribute("class"),
          fuente: estilo.fontFamily, tamano: estilo.fontSize, peso: estilo.fontWeight,
          fondo: estilo.backgroundColor, texto: estilo.color, borde: estilo.borderColor,
        };
      });
    }, selectores);
    const datos = await pagina.evaluate(() => ({
      pantalla: document.querySelector("#page-title").textContent,
      pacientes: document.querySelectorAll(".patient-card").length,
      zoom: getComputedStyle(document.body).zoom,
      escala: devicePixelRatio,
      roboto: document.fonts.check('14px "Sircle Roboto"'),
    }));
    const captura = await pagina.screenshot({ animations: "disabled", fullPage: true });
    const sha256 = createHash("sha256").update(captura).digest("hex");
    if (comparacion) await writeFile(resolve(comparacion, nombre + ".png"), captura);
    pruebas.push({ nombre, datos, controles, sha256 });
    await pagina.close();
  }
  const [referencia, extraccion] = pruebas;
  const informe = { navegador: navegador.version(), viewport: "1280x900", referencia, extraccion };
  if (comparacion) await writeFile(resolve(comparacion, "estilos.json"), JSON.stringify(informe, null, 2) + "\n");
  assert.deepEqual(extraccion.datos, referencia.datos);
  assert.deepEqual(extraccion.controles, referencia.controles);
  assert.equal(extraccion.sha256, referencia.sha256);
  console.log(JSON.stringify({
    navegador: informe.navegador, viewport: informe.viewport, datos: referencia.datos,
    controles_iguales: selectores.length, sha256: referencia.sha256,
  }));
} finally {
  await navegador.close();
}
