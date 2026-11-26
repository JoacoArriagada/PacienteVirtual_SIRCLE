import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const [destino, base, id] = process.argv.slice(2);
assert.ok(base && ["pulso", "respiracion"].includes(id));
if (destino) await mkdir(destino, { recursive: true });
const sitio = "http://127.0.0.1:8080";
const esPulso = id === "pulso";
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
const opciones = { viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 };

try {
  const original = await navegador.newPage(opciones);
  const nueva = await navegador.newPage(opciones);
  for (const pagina of [original, nueva]) await pagina.clock.install();
  const boton = (pagina, name) => pagina.locator("main").getByRole("button", { name, exact: true });
  await original.goto(new URL("../referencia/semana5/Paciente_Virtual.html", import.meta.url).href);
  await boton(original, "Iniciar caso").first().click();
  await boton(original, "Comenzar atención").click();
  await nueva.goto(sitio);
  await nueva.getByLabel("Identidad ficticia").selectOption("estudiante-lr");
  await boton(nueva, "Entrar").click();
  const [alta] = await Promise.all([
    nueva.waitForResponse((r) => r.url() === sitio + "/api/intentos" && r.request().method() === "POST"),
    boton(nueva, "Iniciar caso").click(),
  ]);
  assert.equal(alta.status(), 201);
  const { intento_id: intento } = await alta.json();
  execFileSync("../backend/target/debug/sircle", ["resolver-pre-ensayo", "--db", base, "--intento", intento]);
  await boton(nueva, "Volver a mis pacientes").click();
  await boton(nueva, "Continuar").click();
  const comparacion = {};
  for (const [fuente, pagina] of [["original", original], ["extraccion", nueva]]) {
    await boton(pagina, esPulso ? (fuente === "original" ? "Signos vitales" : "Pulso") : "Respiración").click();
    await boton(pagina, esPulso ? (fuente === "original" ? "Preparar signos vitales" : "Preparar el pulsioxímetro") : "Preparar el conteo").click();
    for (const casilla of await pagina.getByRole("checkbox").all()) await casilla.check();
    const escena = pagina.locator("#device-art");
    if (!esPulso) {
      await boton(pagina, "Contar respiraciones").click();
      await pagina.clock.runFor(1220);
      assert.equal(await boton(pagina, "Contar ciclo").getAttribute("aria-disabled"), "false");
      await boton(pagina, "Contar ciclo").click();
      assert.equal(await boton(pagina, "Contar ciclo").getAttribute("aria-disabled"), "true");
      await pagina.getByText("1 ciclo marcado", { exact: true }).waitFor();
    }
    const recorte = await escena.screenshot({ animations: "disabled" });
    const caja = await escena.boundingBox();
    comparacion[fuente] = {
      ancho: caja.width, alto: caja.height, sha256: createHash("sha256").update(recorte).digest("hex"),
      posicion: await escena.getAttribute("data-positioned"),
      gesto: esPulso ? await escena.locator(".pulse-clip").count() : await escena.locator(".chest-motion").count(),
    };
    if (destino) await pagina.screenshot({ path: resolve(destino, id + "-" + fuente + ".png"), animations: "disabled" });
  }
  assert.equal(comparacion.original.gesto, 1);
  assert.deepEqual(comparacion.original, comparacion.extraccion);

  if (esPulso) {
    await boton(nueva, "Medir pulso").click();
  } else {
    for (let marcado = 0; marcado < 2; marcado++) {
      await nueva.clock.runFor(1150);
      await boton(nueva, "Contar ciclo").click();
    }
    await nueva.getByText("3 ciclos marcados", { exact: true }).waitFor();
  }
  const ruta = sitio + "/api/intentos/" + intento + "/mediciones";
  const [respuesta] = await Promise.all([
    nueva.waitForResponse((r) => r.url() === ruta && r.request().method() === "POST"),
    nueva.clock.fastForward(27000),
  ]);
  const lectura = await respuesta.json();
  assert.equal(respuesta.status(), 201, JSON.stringify(lectura));
  assert.equal(lectura.intento_id, intento);
  assert.equal(lectura.instrumento, id);
  assert.equal(lectura.unidad, esPulso ? "lpm" : "resp/min");
  assert.equal(lectura.origen, esPulso ? "Pulsioxímetro simulado" : "Observación y conteo del estudiante");
  assert.ok(lectura.momento.endsWith("Z"));
  assert.equal(lectura.valor.frecuencia, esPulso ? 102 : 26);
  if (!esPulso) assert.equal(lectura.valor.ciclos, 3);
  await nueva.locator("[data-valor]").waitFor();
  if (destino) await nueva.screenshot({ path: resolve(destino, id + "-resultado.png") });
  const [historial] = await Promise.all([
    nueva.waitForResponse(ruta + "?instrumento=" + id),
    boton(nueva, "Histórico").click(),
  ]);
  const historico = await historial.json();
  assert.deepEqual(historico, [lectura]);
  const evidencia = { navegador: navegador.version(), id, intento, comparacion, lectura, historico };
  if (destino) await writeFile(resolve(destino, id + ".json"), JSON.stringify(evidencia, null, 2) + "\n");
  console.log(JSON.stringify(evidencia));
} finally {
  await navegador.close();
}
