import assert from "node:assert/strict";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const [destino] = process.argv.slice(2);
if (destino) await mkdir(destino, { recursive: true });
const sitio = "http://127.0.0.1:" + (process.env.SIRCLE_PUERTO_FRONTEND ?? "8080");
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
const dimensiones = { viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 };
const boton = (pagina, name) => pagina.locator("main").getByRole("button", { name, exact: true });

async function abrir(identidad) {
  const pagina = await navegador.newPage(dimensiones);
  await pagina.clock.install();
  await pagina.goto(sitio);
  await pagina.getByLabel("Identidad ficticia").selectOption(identidad);
  await boton(pagina, "Entrar").click();
  const respuesta = await accion(pagina, "POST", sitio + "/api/intentos", "Iniciar caso", 201);
  const id = respuesta.intento_id;
  const ruta = sitio + "/api/intentos/" + id;
  const pendiente = await (await pagina.request.get(ruta)).json();
  assert.equal(pendiente.apuntes, undefined);
  assert.equal(pendiente.evidencias, undefined);
  await boton(pagina, "Omitir encuesta").click();
  await pagina.getByRole("heading", { name: "Atender y conversar" }).waitFor();
  return { pagina, id, ruta };
}

async function accion(pagina, metodo, url, control, http) {
  const [respuesta] = await Promise.all([
    pagina.waitForResponse((r) => r.url() === url && r.request().method() === metodo),
    boton(pagina, control).click(),
  ]);
  assert.equal(respuesta.status(), http);
  return respuesta.json();
}

async function guardarApunte({ pagina, ruta }, texto, autor) {
  await pagina.getByRole("tab", { name: "Notas" }).click();
  await pagina.getByLabel("Añadir un punto").fill(texto);
  const apunte = await accion(pagina, "PUT", ruta + "/razonamiento", "Añadir punto", 200);
  assert.equal(apunte.intencion, texto);
  assert.equal(apunte.autor_id, autor);
  await pagina.locator(`[data-apunte-id="${apunte.apunte_id}"]`).getByText(texto, { exact: true }).waitFor();
  return apunte;
}

try {
  const a = await abrir("estudiante-lr");
  const apunteA = await guardarApunte(a, "solicitaría el ECG", "estudiante-lr");
  const antes = await a.pagina.request.get(a.ruta + "/ordenes/ecg-inicial");
  assert.equal(antes.status(), 404);
  const reservado = await antes.json();
  assert.doesNotMatch(JSON.stringify(reservado), /taquicardia sinusal/i);
  const vistaA = await (await a.pagina.request.get(a.ruta)).json();
  assert.deepEqual(vistaA.ordenes, []);
  assert.deepEqual(vistaA.apuntes, [apunteA]);
  if (destino) await a.pagina.getByRole("tabpanel", { name: "Notas" })
    .screenshot({ path: resolve(destino, "apunte-sin-orden.png") });

  const b = await abrir("estudiante-sin-asignacion");
  assert.notEqual(a.id, b.id);
  const pagina = b.pagina;
  await pagina.getByText("Preguntas de ejemplo", { exact: true }).click();
  await boton(pagina, "¿Desde cuándo comenzó?").click();
  const turno = await accion(pagina, "POST", b.ruta + "/turnos", "Enviar", 201);
  assert.match(turno.respuesta, /dos días/i);
  await boton(pagina, "Presión").click();
  await boton(pagina, "Preparar el brazalete").click();
  for (const casilla of await pagina.getByRole("checkbox").all()) await casilla.check();
  await boton(pagina, "Medir presión").click();
  const [toma] = await Promise.all([
    pagina.waitForResponse((r) => r.url() === b.ruta + "/mediciones" && r.request().method() === "POST"),
    pagina.clock.fastForward(21000),
  ]);
  assert.equal(toma.status(), 201);
  const medicion = await toma.json();
  await accion(pagina, "PUT", b.ruta + "/mediciones/" + medicion.medicion_id + "/ficha", "Registrar en ficha", 200);
  await boton(pagina, "Volver a la atención").click();
  await pagina.getByRole("tab", { name: "Adicionales" }).click();
  await boton(pagina, "Solicitar ECG inicial").click();
  const orden = await accion(pagina, "POST", b.ruta + "/ordenes", "Confirmar orden", 201);
  const antesDeConsulta = await (await pagina.request.get(b.ruta)).json();
  assert.ok(antesDeConsulta.evidencias.every((evidencia) => evidencia.tipo !== "informe"));
  const informe = await accion(pagina, "GET", b.ruta + "/ordenes/" + orden.orden_id, "Consultar informe", 200);
  assert.equal(informe.resultado, "taquicardia sinusal");
  const apunteB = await guardarApunte(
    b, "Interpreto la presión junto con lo que refirió LR.", "estudiante-sin-asignacion",
  );

  await boton(pagina, "Volver a mis pacientes").click();
  const vistaB = await accion(pagina, "GET", b.ruta, "Continuar", 200);
  assert.deepEqual(vistaB.apuntes, [apunteB]);
  assert.equal(vistaB.intento_id, b.id);
  const clasificacion = Object.fromEntries(vistaB.evidencias.map((dato) => [dato.categoria, dato]));
  assert.equal(vistaB.evidencias.length, 5);
  assert.ok(vistaB.evidencias.every((dato) => dato.intento_id === b.id && dato.autor));
  assert.equal(clasificacion.declaracion.origen_id, turno.turno_id);
  assert.equal(clasificacion.declaracion.texto, turno.respuesta);
  assert.equal(clasificacion.declaracion.autor, "LR");
  assert.equal(clasificacion.medicion.origen_id, medicion.medicion_id);
  assert.deepEqual(clasificacion.medicion.valor, { sistolica: 148, diastolica: 85 });
  assert.equal(clasificacion.medicion.unidad, "mmHg");
  assert.equal(clasificacion.orientacion.origen_id, vistaB.ayuda.orientacion_id);
  assert.equal(clasificacion.orientacion.texto, vistaB.ayuda.texto);
  assert.equal(clasificacion.orientacion.autor, "docente");
  assert.equal(clasificacion.informe.origen_id, informe.informe_id);
  assert.equal(clasificacion.informe.texto, informe.resultado);
  assert.equal(clasificacion.interpretacion.origen_id, apunteB.apunte_id);
  assert.equal(clasificacion.interpretacion.autor, apunteB.autor_id);
  assert.equal(clasificacion.interpretacion.texto, apunteB.intencion);
  assert.equal((await pagina.request.get(a.ruta)).status(), 403);

  await pagina.getByRole("tab", { name: "Evidencia" }).click();
  const lista = pagina.locator("ul[data-evidencias]");
  for (const [categoria, dato] of [
    ["Declaración", clasificacion.declaracion],
    ["Presión arterial", clasificacion.medicion],
    ["Orientación", clasificacion.orientacion],
    ["Informe", clasificacion.informe],
    ["Interpretación", clasificacion.interpretacion],
  ]) {
    await lista.locator(`li[data-origen-id="${dato.origen_id}"]`).getByText(categoria, { exact: true }).waitFor();
  }
  if (destino) {
    await lista.screenshot({ path: resolve(destino, "evidencias-reabiertas.png") });
    const registro = {
      navegador: navegador.version(), intentoA: { id: a.id, apunte: apunteA, ordenes: vistaA.ordenes, reservado },
      intentoB: { id: b.id, turno, medicion, orden, informe, apunte: apunteB, evidencias: vistaB.evidencias },
    };
    await writeFile(resolve(destino, "evidencias.json"), JSON.stringify(registro, null, 2) + "\n");
  }
  console.log(JSON.stringify({ intentoSinOrden: a.id, intentoReabierto: b.id, evidencias: vistaB.evidencias.length }));
} finally {
  await navegador.close();
}
