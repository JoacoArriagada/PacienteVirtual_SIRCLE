import assert from "node:assert/strict";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const [destino] = process.argv.slice(2);
if (destino) await mkdir(destino, { recursive: true });
const sitio = "http://127.0.0.1:" + (process.env.SIRCLE_PUERTO_FRONTEND ?? "8080");
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
const escena = { viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 };
const boton = (pagina, name) => pagina.locator("main").getByRole("button", { name, exact: true });
const observacion = async (respuesta) => ({ http: respuesta.status(), cuerpo: await respuesta.json() });

try {
  const pagina = await navegador.newPage(escena);
  await pagina.goto(sitio);
  await pagina.getByLabel("Identidad ficticia").selectOption("estudiante-lr");
  await boton(pagina, "Entrar").click();
  const [alta] = await Promise.all([
    pagina.waitForResponse((r) => r.url() === sitio + "/api/intentos" && r.request().method() === "POST"),
    boton(pagina, "Iniciar caso").click(),
  ]);
  assert.equal(alta.status(), 201);
  const { intento_id: id } = await alta.json();
  const intento = sitio + "/api/intentos/" + id;
  const ordenes = intento + "/ordenes";
  const alias = ordenes + "/ecg-inicial";
  const pre = await observacion(await pagina.request.get(alias));
  assert.equal(pre.http, 404);
  assert.doesNotMatch(JSON.stringify(pre), /taquicardia sinusal/i);
  await boton(pagina, "Omitir encuesta").click();

  await pagina.getByRole("tab", { name: "Adicionales" }).click();
  const panel = pagina.getByRole("tabpanel", { name: "Adicionales" });
  const captura = async (nombre) => {
    if (destino) await panel.screenshot({ path: resolve(destino, nombre + ".png") });
  };
  const consultar = boton(pagina, "Consultar informe");
  const [antes] = await Promise.all([pagina.waitForResponse(alias), consultar.click()]);
  const reservado = await observacion(antes);
  assert.deepEqual(reservado, pre);
  await panel.getByText(reservado.cuerpo.mensaje, { exact: true }).waitFor();
  await captura("reservado");
  assert.equal(await panel.getByRole("heading", { name: "Informe del ECG inicial" }).count(), 0);

  const solicitar = boton(pagina, "Solicitar ECG inicial");
  await solicitar.click();
  await boton(pagina, "Cancelar orden").click();
  assert.deepEqual(await observacion(await pagina.request.get(alias)), reservado);
  const sinOrden = await observacion(await pagina.request.get(intento));
  assert.deepEqual(sinOrden.cuerpo.ordenes, []);
  const datos = { examen: "ecg-inicial", accion: "confirmar" };
  const sinConfirmar = await observacion(await pagina.request.post(ordenes, {
    headers: { "Idempotency-Key": "no-confirmada" }, data: { ...datos, accion: "solicitar" },
  }));
  assert.equal(sinConfirmar.http, 422);
  assert.deepEqual(await observacion(await pagina.request.get(alias)), reservado);

  await solicitar.click();
  await captura("confirmacion");
  const [creacion] = await Promise.all([
    pagina.waitForResponse((r) => r.url() === ordenes && r.request().method() === "POST"),
    boton(pagina, "Confirmar orden").click(),
  ]);
  const creada = await observacion(creacion);
  assert.equal(creada.http, 201, JSON.stringify(creada));
  assert.equal(creada.cuerpo.intento_id, id);
  assert.equal(creada.cuerpo.examen, datos.examen);
  assert.equal(creada.cuerpo.fase, "inicial");
  assert.equal(creada.cuerpo.resultado, undefined);
  const clave = creacion.request().headers()["idempotency-key"];
  assert.ok(clave);
  const ruta = ordenes + "/" + creada.cuerpo.orden_id;
  const [primeraConsulta] = await Promise.all([pagina.waitForResponse(ruta), consultar.click()]);
  const informe = await observacion(primeraConsulta);
  assert.equal(informe.http, 200);
  assert.equal(informe.cuerpo.resultado, "taquicardia sinusal");
  assert.equal(informe.cuerpo.orden_id, creada.cuerpo.orden_id);
  assert.equal(informe.cuerpo.intento_id, id);
  assert.match(informe.cuerpo.informe_id, /^[a-f0-9]{32}$/);
  await panel.getByText(informe.cuerpo.resultado, { exact: true }).waitFor();
  assert.equal(await panel.locator("[data-informe] img").count(), 0);
  await captura("informe");

  const reenviar = async (referencia) => observacion(await pagina.request.post(ordenes, {
    headers: { "Idempotency-Key": referencia }, data: datos,
  }));
  const reenvio = await reenviar(clave);
  const otroEnvio = await reenviar("repite-con-clave-nueva");
  assert.deepEqual(reenvio, { http: 200, cuerpo: creada.cuerpo });
  assert.deepEqual(otroEnvio, reenvio);
  const despues = await observacion(await pagina.request.get(intento));
  assert.deepEqual(despues.cuerpo.ordenes, [creada.cuerpo]);

  await boton(pagina, "Volver a mis pacientes").click();
  await boton(pagina, "Continuar").click();
  await pagina.getByRole("tab", { name: "Adicionales" }).click();
  assert.equal(await solicitar.isDisabled(), true);
  const [nuevaConsulta] = await Promise.all([pagina.waitForResponse(ruta), consultar.click()]);
  const reanudado = await observacion(nuevaConsulta);
  assert.deepEqual(reanudado, informe);
  await panel.getByText(informe.cuerpo.resultado, { exact: true }).waitFor();

  const observaciones = { navegador: navegador.version(), escena, intento: id, pre, reservado,
    sinConfirmar, ordenesAntes: sinOrden.cuerpo.ordenes, creada, informe, reenvio, otroEnvio,
    ordenesDespues: despues.cuerpo.ordenes, reanudado };
  if (destino) await writeFile(resolve(destino, "orden-ecg.json"), JSON.stringify(observaciones, null, 2) + "\n");
  console.log(JSON.stringify({ intento: id, orden: creada.cuerpo.orden_id, informe: informe.cuerpo.informe_id }));
} finally {
  await navegador.close();
}
