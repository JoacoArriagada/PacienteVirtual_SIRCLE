import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const [destino, base] = process.argv.slice(2);
assert.ok(base);
if (destino) await mkdir(destino, { recursive: true });
const sitio = "http://127.0.0.1:8080";
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
const opciones = { viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 };
const boton = (pagina, name) => pagina.locator("main").getByRole("button", { name, exact: true });
try {
  const original = await navegador.newPage(opciones);
  const nueva = await navegador.newPage(opciones);
  await nueva.clock.install();
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
  const ruta = "/api/intentos/" + intento;
  const casos = [
    { instrumento: "inspeccion", boton: "Inspección", preparar: "Preparar la inspección",
      iniciar: "Inspeccionar", zona: "frontal", etiqueta: "Tórax anterior", habilitada: true },
    { instrumento: "inspeccion", boton: "Inspección", preparar: "Preparar la inspección",
      iniciar: "Inspeccionar", zona: "posterior", etiqueta: "Tórax posterior", habilitada: false },
    { instrumento: "palpacion", boton: "Palpación", preparar: "Preparar la palpación",
      iniciar: "Palpar", zona: "central", etiqueta: "Tórax anterior", habilitada: true },
    { instrumento: "palpacion", boton: "Palpación", preparar: "Preparar la palpación",
      iniciar: "Palpar", zona: "extremidad", etiqueta: "Pantorrilla derecha", habilitada: false },
  ];
  const registros = [];
  for (const caso of casos) {
    const estados = {};
    for (const [nombre, pagina] of [["original", original], ["extraccion", nueva]]) {
      await boton(pagina, caso.boton).click();
      await boton(pagina, caso.preparar).click();
      const iniciar = boton(pagina, caso.iniciar);
      for (const casilla of await pagina.locator("main").getByRole("checkbox").all()) await casilla.check();
      assert.equal(await iniciar.isDisabled(), true);
      await boton(pagina, caso.etiqueta).click();
      assert.equal(await iniciar.isEnabled(), true);
      const escena = pagina.locator("#device-art");
      assert.equal(await escena.locator(".exam-target").count(), 1);
      assert.equal(await boton(pagina, caso.etiqueta).getAttribute("aria-pressed"), "true");
      const foto = await escena.screenshot({ animations: "disabled" });
      const caja = await escena.boundingBox();
      estados[nombre] = {
        ancho: caja.width, alto: caja.height, huella: createHash("sha256").update(foto).digest("hex"),
      };
      if (destino) await pagina.screenshot({ path: resolve(destino, caso.zona + "-" + nombre + ".png"), animations: "disabled" });
    }
    assert.deepEqual(estados.original, estados.extraccion);
    await boton(original, "Volver a la atención").click();
    await boton(nueva, caso.iniciar).click();
    const [respuesta] = await Promise.all([
      nueva.waitForResponse((r) => r.url() === sitio + ruta + "/exploraciones" && r.request().method() === "POST"),
      nueva.clock.fastForward(8000),
    ]);
    const acuse = await respuesta.json();
    assert.equal(respuesta.status(), 201, JSON.stringify(acuse));
    assert.equal(acuse.intento_id, intento);
    assert.equal(acuse.instrumento, caso.instrumento);
    assert.equal(acuse.zona, caso.zona);
    assert.equal(acuse.etiqueta, caso.etiqueta);
    assert.ok(acuse.momento.endsWith("Z"));
    if (caso.habilitada) {
      assert.equal(acuse.estado, "habilitado");
      assert.ok(acuse.hallazgo.hallazgo_id);
      assert.notEqual(acuse.exploracion_id, acuse.hallazgo.hallazgo_id);
      await nueva.getByText(acuse.hallazgo.texto, { exact: true }).waitFor();
    } else {
      assert.equal(acuse.estado, "pendiente");
      assert.equal(acuse.hallazgo, undefined);
      await nueva.getByText("Hallazgo pendiente", { exact: true }).waitFor();
    }
    if (destino) await nueva.screenshot({ path: resolve(destino, caso.zona + "-resultado.png"), animations: "disabled" });
    registros.push({ zona: caso.zona, estados, acuse });
    await boton(nueva, "Volver a la atención").click();
  }
  const vista = await nueva.evaluate(async (ruta) => (await fetch(ruta)).json(), ruta);
  assert.deepEqual(vista.exploraciones, registros.map((fila) => fila.acuse));
  assert.equal(vista.exploraciones.filter((e) => e.hallazgo?.hallazgo_id).length, 2);
  if (destino) {
    const datos = { navegador: navegador.version(), intento, registros };
    await writeFile(resolve(destino, "acciones.json"), JSON.stringify(datos, null, 2) + "\n");
  }
  console.log(JSON.stringify({ intento, acciones: vista.exploraciones.length, hallazgos: vista.exploraciones.filter((e) => e.hallazgo).length }));
} finally {
  await navegador.close();
}
