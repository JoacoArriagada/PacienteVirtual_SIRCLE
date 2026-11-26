import assert from "node:assert/strict";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const datos = new URL("../../specs/001-simulacion-clinica/evidencias/h1/T020/persistencia.json", import.meta.url);
const previo = JSON.parse(await readFile(datos, "utf8")).despues_de_reabrir;
const frontend = "http://127.0.0.1:4173";
const backend = "http://127.0.0.1:8081";
const pruebas = process.argv[2];
if (pruebas) await mkdir(pruebas, { recursive: true });
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
const dimensiones = { viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 };

try {
  const pagina = await navegador.newPage(dimensiones);
  await pagina.route(frontend + "/api/**", async (ruta) => {
    const url = ruta.request().url().replace(frontend, backend);
    const respuesta = await ruta.fetch({ url });
    await ruta.fulfill({ response: respuesta });
  });
  await pagina.goto(frontend + "/aplicacion");
  await pagina.getByRole("heading", { name: "Entrar a SIRCLE" }).waitFor();

  async function entrar() {
    await pagina.getByLabel("Identidad ficticia").selectOption("estudiante-lr");
    const [respuesta] = await Promise.all([
      pagina.waitForResponse(frontend + "/api/actividades"),
      pagina.getByRole("button", { name: "Entrar", exact: true }).click(),
    ]);
    assert.equal(respuesta.status(), 200);
    assert.equal((await respuesta.json())[0].intento_id, previo.intento_id);
    await pagina.getByRole("button", { name: "En curso", exact: true }).click();
  }

  async function continuar() {
    const [respuesta] = await Promise.all([
      pagina.waitForResponse(frontend + "/api/intentos/" + previo.intento_id),
      pagina.getByRole("button", { name: "Continuar", exact: true }).click(),
    ]);
    assert.equal(respuesta.status(), 200);
    await pagina.getByRole("heading", { name: "Práctica de LR" }).waitFor();
    return respuesta.json();
  }

  await entrar();
  await pagina.evaluate(() => document.fonts.ready);
  if (pruebas) await pagina.screenshot({ path: resolve(pruebas, "en-curso.png"), fullPage: true });
  const selectores = ["#page-title", ".catalog-toolbar", "#patient-search", ".patient-card", ".card-cta .button"];
  const apariencia = (pagina) => pagina.evaluate((selectores) => selectores.map((selector) => {
    const el = document.querySelector(selector);
    const estilo = getComputedStyle(el);
    return {
      selector, clase: el.className, fuente: estilo.fontFamily, tamano: estilo.fontSize,
      fondo: estilo.backgroundColor, ancho: el.getBoundingClientRect().width,
    };
  }), selectores);
  const miPantalla = await apariencia(pagina);
  const referencia = await navegador.newPage(dimensiones);
  await referencia.goto(new URL("../referencia/semana5/Paciente_Virtual.html", import.meta.url).href);
  await referencia.locator("#catalog-cards .patient-card").first().waitFor();
  await referencia.locator("#patient-search").fill("LR");
  await referencia.locator("#catalog-cards .patient-card").last().waitFor();
  await referencia.evaluate(() => document.fonts.ready);
  const original = await apariencia(referencia);
  assert.deepEqual(miPantalla, original);
  if (pruebas) await referencia.screenshot({ path: resolve(pruebas, "referencia.png"), fullPage: true });

  const antes = await continuar();
  const [salida] = await Promise.all([
    pagina.waitForResponse((res) => res.url() === frontend + "/api/sesiones" && res.request().method() === "DELETE"),
    pagina.getByRole("button", { name: "Salir", exact: true }).click(),
  ]);
  assert.equal(salida.status(), 204);
  await entrar();
  const despues = await continuar();
  assert.equal(antes.intento_id, previo.intento_id);
  assert.equal(antes.version_caso, previo.version_caso);
  assert.deepEqual(antes.turnos, previo.turnos);
  assert.deepEqual(despues, antes);
  if (pruebas) await pagina.screenshot({ path: resolve(pruebas, "reabierto.png"), fullPage: true });
  const evidencia = { navegador: navegador.version(), dimensiones, original, implementacion: miPantalla, antes, despues };
  if (pruebas) await writeFile(resolve(pruebas, "respuestas.json"), JSON.stringify(evidencia, null, 2) + "\n");
  console.log(JSON.stringify({ intento_id: despues.intento_id, version: despues.version_caso, turnos: despues.turnos.length, controles: original.length }));
} finally {
  await navegador.close();
}
