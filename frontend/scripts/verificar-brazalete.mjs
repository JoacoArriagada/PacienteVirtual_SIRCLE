import assert from "node:assert/strict";
import { mkdir } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const origen = "http://127.0.0.1:4173";
const directorio = process.argv[2];
if (directorio) await mkdir(directorio, { recursive: true });
await assert.rejects(fetch("http://127.0.0.1:8081/api/estado"), (error) => error.cause?.code === "ECONNREFUSED");

const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
try {
  const pagina = await navegador.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
  const brazo = pagina.locator("#brazo");
  const estado = pagina.locator("#presion-estado");
  const historial = pagina.locator("#presion-historico > li");
  const preparar = pagina.getByRole("button", { name: "Preparar brazalete" });
  const colocar = pagina.getByRole("button", { name: "Colocar brazalete" });
  const tomar = pagina.getByRole("button", { name: "Tomar presión" });
  const registrar = pagina.getByRole("button", { name: "Registrar en ficha" });
  const solicitudesApi = [];
  pagina.on("request", (peticion) => {
    if (new URL(peticion.url()).pathname.startsWith("/api/")) solicitudesApi.push(peticion.url());
  });

  await pagina.goto(origen);
  await preparar.waitFor();
  const area = await brazo.boundingBox();
  const capturar = async (nombre) => {
    assert.deepEqual(await brazo.boundingBox(), area);
    assert.ok(await brazo.evaluate(async (imagen) => {
      await imagen.decode();
      return imagen.naturalWidth > 0;
    }));
    if (directorio) await pagina.screenshot({ path: resolve(directorio, nombre + ".png") });
  };

  assert.equal(await tomar.isDisabled(), true);
  await preparar.click();
  assert.match(await estado.innerText(), /Brazalete preparado/);
  await capturar("01-preparado");
  await colocar.click();
  assert.match(await estado.innerText(), /Brazalete colocado/);
  await capturar("02-colocado");
  const [respuesta] = await Promise.all([
    pagina.waitForResponse(origen + "/ejemplos/presion.json"),
    tomar.click(),
  ]);
  assert.equal(respuesta.status(), 200);
  const lectura = await respuesta.json();
  await pagina.getByText("148/85 mmHg · " + lectura.fuente).waitFor();
  assert.equal(await historial.count(), 0);
  await capturar("03-resultado");
  await registrar.click();
  const primerRegistro = await historial.count();
  await registrar.click();
  const segundoRegistro = await historial.count();
  assert.equal(primerRegistro, 1);
  assert.equal(segundoRegistro, 1);
  assert.equal(await historial.first().getAttribute("data-medicion-id"), lectura.medicion_id);
  assert.deepEqual(solicitudesApi, []);
  await capturar("04-historico");

  console.log(JSON.stringify({
    navegador: "Chromium " + navegador.version(),
    backend: "conexión rechazada",
    resolucion: "1280x900",
    escala: await pagina.evaluate(() => devicePixelRatio),
    encuadre_brazo: area,
    lectura,
    primerRegistro,
    segundoRegistro,
    solicitudesApi,
  }));
} finally {
  await navegador.close();
}
