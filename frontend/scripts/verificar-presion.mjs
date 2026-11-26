import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const [destino, base] = process.argv.slice(2);
assert.ok(base, "Indica la ruta de la base temporal.");
if (destino) await mkdir(destino, { recursive: true });
const sitio = "http://127.0.0.1:8080";
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
try {
  const pagina = await navegador.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
  await pagina.clock.install();
  await pagina.goto(sitio);
  const boton = (name) => pagina.getByRole("button", { name, exact: true });
  await pagina.getByLabel("Identidad ficticia").selectOption("estudiante-lr");
  await boton("Entrar").click();
  const [alta] = await Promise.all([
    pagina.waitForResponse((r) => r.url() === sitio + "/api/intentos" && r.request().method() === "POST"),
    boton("Iniciar caso").click(),
  ]);
  assert.equal(alta.status(), 201);
  const { intento_id: id } = await alta.json();
  const ruta = "/api/intentos/" + id;
  const peticiones = [];
  pagina.on("request", (r) => {
    if (r.method() === "POST" && r.url() === sitio + ruta + "/mediciones") peticiones.push(r);
  });
  execFileSync("../backend/target/debug/sircle", ["resolver-pre-ensayo", "--db", base, "--intento", id]);
  await boton("Volver a mis pacientes").click();
  await boton("Continuar").click();
  await boton("Presión").click();

  async function preparar(nombre = "Preparar el brazalete") {
    await boton(nombre).click();
    for (const casilla of await pagina.getByRole("checkbox").all()) await casilla.check();
    await boton("Medir presión").click();
  }
  async function capturar(nombre) {
    if (destino) await pagina.screenshot({ path: resolve(destino, nombre + ".png"), animations: "disabled" });
  }

  await preparar();
  await pagina.clock.runFor(1000);
  await capturar("toma-en-curso");
  await boton("Cancelar toma").click();
  await pagina.clock.fastForward(21000);
  const [cancelacion] = await Promise.all([
    pagina.waitForResponse(sitio + ruta + "/mediciones?instrumento=presion"),
    boton("Histórico").click(),
  ]);
  assert.deepEqual(await cancelacion.json(), []);
  assert.equal(peticiones.length, 0);
  await pagina.getByText("Aún no hay tomas.", { exact: true }).waitFor();
  await capturar("historico-tras-cancelar");
  await boton("Preparar otra toma").click();
  await preparar();
  const [completa] = await Promise.all([
    pagina.waitForResponse((r) => r.url() === sitio + ruta + "/mediciones" && r.request().method() === "POST"),
    pagina.clock.fastForward(21000),
  ]);
  assert.equal(completa.status(), 201);
  const lectura = await completa.json();
  assert.equal(lectura.en_ficha, false);
  assert.equal(peticiones.length, 1);
  await pagina.locator(".metric dd strong").filter({ hasText: /^148$/ }).waitFor();
  await pagina.locator(".metric dd strong").filter({ hasText: /^85$/ }).waitFor();
  await capturar("resultado");

  const acuses = [];
  const ficha = sitio + ruta + "/mediciones/" + lectura.medicion_id + "/ficha";
  for (let registro = 0; registro < 2; registro++) {
    const [respuesta] = await Promise.all([
      pagina.waitForResponse(ficha),
      boton("Registrar en ficha").click(),
    ]);
    assert.equal(respuesta.status(), 200);
    acuses.push({
      http: respuesta.status(),
      clave: respuesta.request().headers()["idempotency-key"],
      cuerpo: await respuesta.json(),
    });
  }
  assert.deepEqual(acuses[0], acuses[1]);
  assert.equal(acuses[0].cuerpo.medicion_id, lectura.medicion_id);
  assert.equal(acuses[0].cuerpo.en_ficha, true);
  await capturar("registro-en-ficha");
  const [consulta] = await Promise.all([
    pagina.waitForResponse(sitio + ruta + "/mediciones?instrumento=presion"),
    boton("Histórico").click(),
  ]);
  const historico = await consulta.json();
  assert.deepEqual(historico, [acuses[0].cuerpo]);
  await boton("Tabla").click();
  await pagina.locator("tr[data-medicion-id]").waitFor();
  await capturar("historico");

  const [reabierto] = await Promise.all([
    pagina.waitForResponse(sitio + ruta),
    boton("Volver a la atención").click(),
  ]);
  const intento = await reabierto.json();
  assert.deepEqual(intento.mediciones, historico);
  assert.deepEqual(intento.ficha.hallazgos, historico);
  await pagina.getByText("Ficha inicial", { exact: true }).click();
  assert.equal(await pagina.locator("[data-hallazgos] li[data-medicion-id]").count(), 1);
  assert.equal(await pagina.locator("[data-hallazgos] li").getAttribute("data-medicion-id"), lectura.medicion_id);
  await capturar("ficha");
  const informe = {
    id, navegador: navegador.version(), cancelada: { peticiones: 0, historico: [] },
    completa: { http: completa.status(), clave: completa.request().headers()["idempotency-key"], lectura },
    acuses, historico, ficha: intento.ficha,
  };
  if (destino) await writeFile(resolve(destino, "registro.json"), JSON.stringify(informe, null, 2) + "\n");
  console.log(JSON.stringify({ id, medicion: lectura.medicion_id, historico: historico.length, ficha: intento.ficha.hallazgos.length }));
} finally {
  await navegador.close();
}
