import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const destino = process.argv[2];
const base = process.argv[3];
assert.ok(base, "Indica la ruta de una base de ensayo separada.");
if (destino) await mkdir(destino, { recursive: true });
const backend = "http://127.0.0.1:8081";
const frontend = "http://127.0.0.1:4173";
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
const dimensiones = { viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 };
try {
  const pagina = await navegador.newPage(dimensiones);
  await pagina.route(frontend + "/api/**", async (ruta) => {
    const respuesta = await ruta.fetch({ url: ruta.request().url().replace(frontend, backend) });
    await ruta.fulfill({ response: respuesta });
  });
  await pagina.goto(frontend + "/aplicacion");
  await pagina.getByLabel("Identidad ficticia").selectOption("estudiante-lr");
  await pagina.getByRole("button", { name: "Entrar", exact: true }).click();
  const [alta] = await Promise.all([
    pagina.waitForResponse((r) => r.url() === frontend + "/api/intentos" && r.request().method() === "POST"),
    pagina.getByRole("button", { name: "Iniciar caso", exact: true }).click(),
  ]);
  assert.equal(alta.status(), 201);
  const intento = await alta.json();
  const id = intento.intento_id;
  const ruta = "/api/intentos/" + id;
  const pendiente = await pagina.evaluate(async (ruta) => {
    const respuesta = await fetch(ruta);
    if (!respuesta.ok) throw new Error("HTTP " + respuesta.status);
    return respuesta.json();
  }, ruta);
  for (const seccion of ["introduccion", "ficha", "indicaciones", "ayuda", "turnos", "mediciones"]) {
    assert.equal(pendiente[seccion], undefined, seccion);
  }
  await pagina.getByText("Pre pendiente.", { exact: true }).waitFor();
  assert.equal(await pagina.locator("summary").count(), 0);
  assert.equal(await pagina.getByText("Falta de aire", { exact: true }).count(), 0);
  if (destino) await pagina.screenshot({ path: resolve(destino, "pre-pendiente.png"), fullPage: true });

  execFileSync("../backend/target/debug/sircle", ["resolver-pre-ensayo", "--db", base, "--intento", id]);
  await pagina.getByRole("button", { name: "Volver a mis pacientes", exact: true }).click();
  const [resuelta] = await Promise.all([
    pagina.waitForResponse(frontend + ruta),
    pagina.getByRole("button", { name: "Continuar", exact: true }).click(),
  ]);
  assert.equal(resuelta.status(), 200);
  const contenido = await resuelta.json();
  assert.equal(contenido.intento_id, id);
  assert.equal(contenido.pre, "omitido");
  assert.equal(contenido.fase, "inicial");
  assert.equal(contenido.ayuda.origen, "docente");
  assert.deepEqual(contenido.mediciones, []);
  await pagina.getByText("Falta de aire", { exact: true }).waitFor();
  for (const parte of ["Ficha inicial", "Indicaciones", "Ayuda"]) {
    await pagina.locator("main").getByText(parte, { exact: true }).click();
  }
  await pagina.getByText("148/85 mmHg", { exact: true }).waitFor();
  await pagina.getByText(contenido.indicaciones[0], { exact: true }).waitFor();
  await pagina.getByRole("heading", { name: "Orientación docente", exact: true }).waitFor();
  assert.equal(await pagina.locator("[data-ayuda]").getAttribute("data-origen"), "docente");
  if (destino) await pagina.screenshot({ path: resolve(destino, "pre-resuelto.png"), fullPage: true });
  const evidencia = { navegador: navegador.version(), dimensiones, pendiente, contenido };
  if (destino) await writeFile(resolve(destino, "respuestas.json"), JSON.stringify(evidencia, null, 2) + "\n");
  console.log(JSON.stringify({ id, inicial: pendiente.pre, final: contenido.pre, origen: contenido.ayuda.origen }));
} finally {
  await navegador.close();
}
