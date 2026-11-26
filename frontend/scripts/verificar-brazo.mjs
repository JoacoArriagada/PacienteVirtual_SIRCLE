import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const destino = process.argv[2];
if (destino) await mkdir(destino, { recursive: true });
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
const viewport = { width: 1280, height: 900 };
const opciones = { viewport, deviceScaleFactor: 1 };

try {
  const original = await navegador.newPage(opciones);
  const extraccion = await navegador.newPage(opciones);
  await original.goto(new URL("../referencia/semana5/Paciente_Virtual.html", import.meta.url).href);
  await original.getByRole("button", { name: "Iniciar caso", exact: true }).first().click();
  await original.getByRole("button", { name: "Comenzar atención", exact: true }).click();
  await extraccion.goto("http://127.0.0.1:8080");
  await extraccion.getByLabel("Identidad ficticia").selectOption("estudiante-lr");
  await extraccion.getByRole("button", { name: "Entrar", exact: true }).click();
  await extraccion.getByRole("button", { name: "Continuar", exact: true }).click();

  const series = {};
  for (const [fuente, pagina] of [["original", original], ["extraccion", extraccion]]) {
    await pagina.evaluate(() => document.fonts.ready);
    const boton = (name) => pagina.getByRole("button", { name, exact: true });
    const escena = pagina.locator("#device-art");
    const estados = {};
    const capturar = async (nombre, imagen) => {
      const captura = await imagen.screenshot({ animations: "disabled" });
      const caja = await imagen.boundingBox();
      estados[nombre] = {
        ancho: caja.width, alto: caja.height,
        sha256: createHash("sha256").update(captura).digest("hex"),
      };
      if (destino) {
        await pagina.evaluate(() => scrollTo(0, 0));
        await pagina.screenshot({ path: resolve(destino, fuente + "-" + nombre + ".png"), animations: "disabled" });
      }
    };

    await boton("Ver brazo").click();
    await pagina.getByRole("heading", { name: "Vista del brazo · LR", exact: true }).waitFor();
    await capturar("brazo", pagina.locator(".arm-image"));
    await boton("Cerrar ventana").click();

    await boton("Presión").click();
    await boton("Preparar el brazalete").click();
    const medir = boton("Medir presión");
    assert.equal(await medir.isDisabled(), true);
    assert.equal(await escena.getAttribute("data-phase"), "prepare");
    assert.equal(await escena.getAttribute("data-positioned"), "false");
    const controles = pagina.getByRole("region", { name: "Controles del instrumento" });
    const pasos = controles.getByRole("checkbox");
    assert.equal(await pasos.count(), 3);
    estados.pasos = await controles.locator("fieldset label span").allTextContents();
    await capturar("preparacion", escena);

    await pasos.nth(0).check();
    await pasos.nth(1).check();
    assert.equal(await escena.getAttribute("data-positioned"), "true");
    assert.equal(await medir.isDisabled(), true);
    await capturar("colocado", escena);

    await pasos.nth(2).check();
    assert.equal(await medir.isEnabled(), true);
    await capturar("ajustado", escena);
    estados.habilitacion = { preparacion: false, colocado: false, ajustado: true };
    series[fuente] = estados;
  }

  const informe = { navegador: navegador.version(), viewport, escala: 1, zoom: 100, ...series };
  if (destino) await writeFile(resolve(destino, "estados.json"), JSON.stringify(informe, null, 2) + "\n");
  assert.deepEqual(series.extraccion, series.original);
  console.log(JSON.stringify(informe));
} finally {
  await navegador.close();
}
