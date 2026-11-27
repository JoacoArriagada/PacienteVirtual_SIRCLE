import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const [destino, base, modo] = process.argv.slice(2);
assert.ok(base, "Indica la ruta de la base temporal.");
if (destino) await mkdir(destino, { recursive: true });
const sitio = "http://127.0.0.1:8080";
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
const opciones = { viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 };
const boton = (pagina, name) => pagina.locator("main").getByRole("button", { name, exact: true });
try {
  const pagina = await navegador.newPage(opciones);
  const original = await navegador.newPage(opciones);
  await pagina.clock.install();
  await original.goto(new URL("../referencia/semana5/Paciente_Virtual.html", import.meta.url).href);
  await boton(original, "Iniciar caso").first().click();
  await boton(original, "Comenzar atención").click();
  await pagina.goto(sitio);
  await pagina.getByLabel("Identidad ficticia").selectOption("estudiante-lr");
  await boton(pagina, "Entrar").click();
  const [alta] = await Promise.all([
    pagina.waitForResponse((r) => r.url() === sitio + "/api/intentos" && r.request().method() === "POST"),
    boton(pagina, "Iniciar caso").click(),
  ]);
  assert.equal(alta.status(), 201);
  const { intento_id: intento } = await alta.json();
  execFileSync("../backend/target/debug/sircle", ["resolver-pre-ensayo", "--db", base, "--intento", intento]);
  await boton(pagina, "Volver a mis pacientes").click();
  await boton(pagina, "Continuar").click();
  const ruta = "/api/intentos/" + intento;
  const lecturas = {};
  const sensores = modo === "peso" ? [
    { id: "peso", nombre: "Peso", preparar: "Preparar la báscula", medir: "Medir peso",
      clase: ".positioned-photo", unidad: "kg", valor: 64.3, origen: "Báscula de ensayo, Semana 5" },
  ] : [
    { id: "temperatura", nombre: "Temperatura", preparar: "Preparar el termómetro", medir: "Medir temperatura",
      clase: ".thermometer", unidad: "°C", valor: 36.6, origen: "Termómetro frontal simulado" },
    { id: "saturacion", nombre: "Saturación", preparar: "Preparar el sensor", medir: "Medir saturación",
      clase: ".pulse-clip", unidad: "%", valor: 84, origen: "Pulsioxímetro simulado" },
  ];
  for (const sensor of sensores) {
    const comparacion = {};
    for (const [nombre, navegacion] of [["original", original], ["extraccion", pagina]]) {
      await boton(navegacion, sensor.nombre).click();
      await boton(navegacion, sensor.preparar).click();
      assert.equal(await boton(navegacion, sensor.medir).isDisabled(), true);
      const casillas = await navegacion.locator("main").getByRole("checkbox").all();
      for (const [indice, casilla] of casillas.entries()) {
        await casilla.check();
        if (indice + 1 < casillas.length) assert.equal(await boton(navegacion, sensor.medir).isDisabled(), true);
      }
      assert.equal(await boton(navegacion, sensor.medir).isEnabled(), true);
      const escena = navegacion.locator("#device-art");
      assert.equal(await escena.getAttribute("data-positioned"), "true");
      assert.equal(await escena.locator(sensor.clase).count(), 1);
      const recorte = await escena.screenshot({ animations: "disabled" });
      const caja = await escena.boundingBox();
      comparacion[nombre] = {
        ancho: caja.width, alto: caja.height, sha256: createHash("sha256").update(recorte).digest("hex"),
      };
      if (destino) await navegacion.screenshot({ path: resolve(destino, sensor.id + "-" + nombre + ".png"), animations: "disabled" });
    }
    assert.deepEqual(comparacion.original, comparacion.extraccion);
    await boton(original, "Volver a la atención").click();
    await boton(pagina, sensor.medir).click();
    const [resultado] = await Promise.all([
      pagina.waitForResponse((r) => r.url() === sitio + ruta + "/mediciones" && r.request().method() === "POST"),
      pagina.clock.fastForward(12000),
    ]);
    assert.equal(resultado.status(), 201);
    const lectura = await resultado.json();
    assert.equal(lectura.intento_id, intento);
    assert.equal(lectura.instrumento, sensor.id);
    assert.equal(lectura.valor.lectura, sensor.valor);
    assert.equal(lectura.unidad, sensor.unidad);
    assert.equal(lectura.origen, sensor.origen);
    assert.ok(lectura.momento.endsWith("Z"));
    const [registrada] = await Promise.all([
      pagina.waitForResponse(sitio + ruta + "/mediciones/" + lectura.medicion_id + "/ficha"),
      boton(pagina, "Registrar en ficha").click(),
    ]);
    assert.equal(registrada.status(), 200);
    const acuse = await registrada.json();
    assert.equal(acuse.medicion_id, lectura.medicion_id);
    assert.equal(acuse.en_ficha, true);
    lecturas[sensor.id] = { comparacion, lectura, acuse };
    if (destino) await pagina.screenshot({ path: resolve(destino, sensor.id + "-resultado.png") });
    await boton(pagina, "Volver a la atención").click();
  }
  const vista = await pagina.evaluate(async (ruta) => (await fetch(ruta)).json(), ruta);
  assert.equal(vista.mediciones.length, sensores.length);
  assert.deepEqual(vista.ficha.hallazgos, vista.mediciones);
  for (const sensor of sensores) {
    const [consulta] = await Promise.all([
      pagina.waitForResponse(sitio + ruta + "/mediciones?instrumento=" + sensor.id),
      (async () => {
        await boton(pagina, sensor.nombre).click();
        await boton(pagina, "Histórico").click();
      })(),
    ]);
    const historico = await consulta.json();
    assert.deepEqual(historico, [lecturas[sensor.id].acuse]);
    lecturas[sensor.id].historico = historico;
    await boton(pagina, "Volver a la atención").click();
  }
  await pagina.getByRole("button", { name: "Ficha clínica", exact: true }).click();
  for (const sensor of sensores) {
    const id = lecturas[sensor.id].lectura.medicion_id;
    await pagina.locator('[data-hallazgos] [data-medicion-id="' + id + '"]').waitFor();
  }
  if (destino) await pagina.screenshot({ path: resolve(destino, "ficha.png") });
  const prueba = { navegador: navegador.version(), intento, lecturas, ficha: vista.ficha };
  if (destino) await writeFile(resolve(destino, "lecturas.json"), JSON.stringify(prueba, null, 2) + "\n");
  console.log(JSON.stringify({ intento, ids: Object.fromEntries(
    sensores.map((sensor) => [sensor.id, lecturas[sensor.id].lectura.medicion_id]),
  ) }));
} finally {
  await navegador.close();
}
