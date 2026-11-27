import assert from "node:assert/strict";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { setTimeout as esperar } from "node:timers/promises";
import { chromium } from "playwright-core";

const [destino] = process.argv.slice(2);
assert.ok(destino, "Indica el directorio de evidencia de un recorrido nuevo de ensayo.");
const url = new URL(process.env.SIRCLE_MODULO01_URL ?? "http://127.0.0.1:8080");
assert.equal(url.protocol, "http:");
assert.equal(url.hostname, "127.0.0.1");
const sitio = url.origin;
await mkdir(destino, { recursive: true });
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
const boton = (pagina, name) => pagina.getByRole("button", { name, exact: true });
const captura = (pagina, nombre) => pagina.screenshot({ path: resolve(destino, nombre + ".png"), fullPage: true });

async function json(respuesta, codigo = 200) {
  assert.equal(respuesta.status(), codigo, await respuesta.text());
  return respuesta.json();
}

async function api(pagina, ruta, codigo = 200, opciones) {
  return json(await pagina.request.fetch(sitio + "/api" + ruta, opciones), codigo);
}

async function accion(pagina, ruta, metodo, ejecutar, timeout) {
  const [respuesta] = await Promise.all([
    pagina.waitForResponse((r) => r.url() === sitio + "/api" + ruta && r.request().method() === metodo, { timeout }),
    ejecutar(),
  ]);
  return respuesta;
}

async function sesion(identidad) {
  const pagina = await navegador.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
  await pagina.goto(sitio);
  await pagina.getByLabel("Identidad ficticia").selectOption(identidad);
  await json(await accion(pagina, "/sesiones", "POST", () => boton(pagina, "Entrar").click()), 201);
  return pagina;
}

try {
  const estudiante = await sesion("estudiante-lr");
  const estado = await api(estudiante, "/estado");
  assert.equal(estado.base.disponible, true);
  assert.equal(estado.modelo.disponible, true, "Se requiere Ollama real preparado para este ensayo.");
  const actividades = await api(estudiante, "/actividades");
  assert.deepEqual(actividades.map(({ actividad_id, tipo, intento_id }) => ({ actividad_id, tipo, intento_id })),
    [{ actividad_id: "lr-ensayo", tipo: "ensayo", intento_id: null }], "Usa una base de ensayo con un intento nuevo.");
  const alta = await json(await accion(estudiante, "/intentos", "POST", () => boton(estudiante, "Iniciar caso").click()), 201);
  const intento = "/intentos/" + alta.intento_id;
  const anterior = await api(estudiante, intento);
  assert.equal(anterior.actividad_id, "lr-ensayo");
  assert.equal(anterior.introduccion, undefined);
  assert.equal(anterior.ficha, undefined);
  const pre = estudiante.locator("form.encuesta");
  await pre.waitFor();
  assert.equal(await pre.locator("fieldset").count(), 5);
  for (const pregunta of await pre.locator("fieldset").all()) await pregunta.getByRole("radio").first().check();
  const preAcuse = await json(await accion(estudiante, intento + "/encuestas/pre", "PUT",
    () => boton(estudiante, "Enviar respuestas").click()));
  assert.equal(preAcuse.estado, "respondido");
  await estudiante.getByRole("heading", { name: "Atender y conversar", exact: true }).waitFor();
  await boton(estudiante, "Volver a mis pacientes").click();
  await boton(estudiante, "En curso").click();
  const recuperado = await json(await accion(estudiante, intento, "GET", () => boton(estudiante, "Continuar").click()));
  assert.equal(recuperado.intento_id, alta.intento_id);
  assert.equal(recuperado.version_caso, anterior.version_caso);
  assert.equal(recuperado.pre, "respondido");

  const pregunta = "¿Cuándo empezaste a notar que te faltaba el aire?";
  await estudiante.getByLabel("Pregunta al paciente").fill(pregunta);
  const inicio = performance.now();
  const respondida = await accion(estudiante, intento + "/turnos", "POST", () => boton(estudiante, "Enviar").click(), 200_000);
  const latenciaTurnoMs = Math.round(performance.now() - inicio);
  assert.deepEqual(respondida.request().postDataJSON(), { texto: pregunta });
  const turno = await json(respondida, 201);
  assert.equal(turno.hablante, "LR");
  assert.match(turno.respuesta, /hace dos días/i);
  await estudiante.locator(`[data-turno="${turno.turno_id}"]`).getByText(turno.respuesta, { exact: true }).waitFor();
  await captura(estudiante, "entrevista");
  const rutaTraza = intento + "/turnos/" + turno.turno_id + "/traza";
  await api(estudiante, rutaTraza, 403);

  await estudiante.clock.install();
  async function medir(nombre, preparar, iniciar, simulacionMs) {
    await boton(estudiante, nombre).click();
    await boton(estudiante, preparar).click();
    for (const casilla of await estudiante.getByRole("region", { name: "Controles del instrumento" }).getByRole("checkbox").all()) {
      await casilla.check();
    }
    await boton(estudiante, iniciar).click();
    return json(await accion(estudiante, intento + "/mediciones", "POST", () => estudiante.clock.fastForward(simulacionMs)), 201);
  }
  const presion = await medir("Presión", "Preparar el brazalete", "Medir presión", 21_000);
  assert.deepEqual(presion.valor, { sistolica: 148, diastolica: 85 });
  assert.equal(presion.unidad, "mmHg");
  const registro = await json(await accion(estudiante, intento + "/mediciones/" + presion.medicion_id + "/ficha", "PUT",
    () => boton(estudiante, "Registrar en ficha").click()));
  assert.equal(registro.medicion_id, presion.medicion_id);
  await captura(estudiante, "presion");
  await boton(estudiante, "Volver a la atención").click();
  const pulso = await medir("Signos vitales", "Preparar el pulsioxímetro", "Medir pulso", 25_000);
  assert.equal(pulso.valor.frecuencia, 102);
  await boton(estudiante, "Volver a la atención").click();

  const alias = intento + "/ordenes/ecg-inicial";
  await api(estudiante, alias, 404);
  await estudiante.getByRole("tab", { name: "Adicionales" }).click();
  await boton(estudiante, "Solicitar ECG inicial").click();
  const orden = await json(await accion(estudiante, intento + "/ordenes", "POST",
    () => boton(estudiante, "Confirmar orden").click()), 201);
  const informe = await json(await accion(estudiante, intento + "/ordenes/" + orden.orden_id, "GET",
    () => boton(estudiante, "Consultar informe").click()));
  assert.match(informe.resultado, /taquicardia sinusal/i);
  await estudiante.getByText(informe.resultado, { exact: true }).waitFor();

  await estudiante.getByRole("tab", { name: "Hipótesis" }).click();
  await boton(estudiante, "Fundamentar hipótesis").click();
  for (const [grupo, titulo, fundamento, ids] of [
    ["Hipótesis principal", "Broncoespasmo", "La dificultad respiratoria comenzó hace dos días. Falta integrar la auscultación.", [turno.turno_id, presion.medicion_id]],
    ["Alternativa", "Origen cardíaco de la disnea", "El ECG informa taquicardia sinusal. El hallazgo permite contrastar esta alternativa junto con el pulso.", [informe.informe_id, pulso.medicion_id]],
  ]) {
    const formulario = estudiante.getByRole("group", { name: grupo, exact: true });
    await formulario.getByLabel("Interpretación provisional").fill(titulo);
    await formulario.getByLabel("Fundamento y límites").fill(fundamento);
    for (const id of ids) await formulario.locator(`input[value="${id}"]`).check();
  }
  const razonamiento = await json(await accion(estudiante, intento + "/razonamiento", "PUT",
    () => boton(estudiante, "Guardar hipótesis").click()));
  assert.deepEqual(razonamiento.hipotesis.map((h) => [h.rol, h.evidencias.length]), [["principal", 2], ["alternativa", 2]]);
  await estudiante.getByText("Hipótesis guardadas.", { exact: true }).waitFor();
  await boton(estudiante, "Revisar entrega").click();
  const confirmada = await accion(estudiante, intento + "/entrega", "POST", () => boton(estudiante, "Confirmar entrega").click());
  const entrega = await json(confirmada, 201);
  assert.deepEqual(await api(estudiante, intento + "/entrega", 200, {
    method: "POST", data: confirmada.request().postDataJSON(),
    headers: { "Idempotency-Key": confirmada.request().headers()["idempotency-key"] },
  }), entrega);
  await estudiante.getByRole("heading", { name: "Después de entregar" }).waitFor();
  assert.equal(await estudiante.locator("form.encuesta fieldset").count(), 7);
  console.log(JSON.stringify({ intento_id: alta.intento_id, entrega_id: entrega.entrega_id, latencia_turno_ms: latenciaTurnoMs }));

  const docente = await sesion("docente-demo");
  const traza = await api(docente, rutaTraza);
  assert.equal(traza.operacion_id, turno.operacion_id);
  assert.equal(traza.invocacion.modelo, estado.modelo.nombre);
  assert.equal(traza.invocacion.digest, estado.modelo.digest_esperado);
  assert.equal(traza.invocacion.seleccion.hecho_id, "lr-inicio");
  let evaluacionId;
  const inicioEvaluacion = performance.now();
  const listado = "/actividades/lr-ensayo/entregas";
  while (performance.now() - inicioEvaluacion < 580_000) {
    const filas = (await api(docente, listado)).filter((fila) => fila.intento_id === alta.intento_id);
    assert.equal(filas.length, 1);
    assert.equal(filas[0].entrega_id, entrega.entrega_id);
    if (filas[0].estado === "terminado") {
      evaluacionId = filas[0].evaluacion_id;
      break;
    }
    await esperar(2_000);
  }
  assert.ok(evaluacionId, "El trabajo de evaluación no terminó.");
  const rutaEvaluacion = "/evaluaciones/" + evaluacionId;
  const evaluacion = await api(docente, rutaEvaluacion);
  const trazaEvaluacion = await api(docente, rutaEvaluacion + "/traza");
  assert.equal(trazaEvaluacion.invocacion.modelo, estado.modelo.nombre);
  assert.equal(trazaEvaluacion.invocacion.digest, estado.modelo.digest_esperado);
  assert.equal(trazaEvaluacion.entrega.id, entrega.entrega_id);
  const criterio = await api(docente, rutaEvaluacion + "/criterios/valoracion/traza?revision=1");
  assert.ok(criterio.citas.some(({ evidencia }) => evidencia.id === presion.medicion_id || evidencia.id === pulso.medicion_id));
  await docente.reload();
  await boton(docente, "Revisar").click();
  const valoracion = docente.locator("details").filter({ has: docente.locator("summary").filter({ hasText: /^Valoración ·/ }) });
  await valoracion.locator("summary").click();
  await valoracion.getByLabel("Mejora").fill("Relaciona cada lectura registrada con la hipótesis para justificar la decisión.");
  await docente.getByLabel("Motivo de rectificación").fill("Ensayo de revisión docente con los hallazgos registrados.");
  const revision = await json(await accion(docente, rutaEvaluacion, "PATCH", () => boton(docente, "Guardar revisión").click()), 201);
  assert.equal(revision.revision, 2);
  assert.equal(revision.autor_id, "docente-demo");
  const publicacion = await json(await accion(docente, rutaEvaluacion + "/publicacion", "POST",
    () => boton(docente, "Publicar devolución").click()), 201);
  assert.equal(publicacion.revision, 2);
  await docente.getByText("Revisión publicada 2.", { exact: true }).waitFor();
  await captura(docente, "revision-docente");

  assert.deepEqual(await api(estudiante, intento + "/devolucion"), { estado: "post_pendiente" });
  const post = await json(await accion(estudiante, intento + "/encuestas/post", "PUT",
    () => boton(estudiante, "Omitir encuesta").click()));
  assert.equal(post.estado, "omitido");
  const publicada = await api(estudiante, intento + "/devolucion");
  assert.equal(publicada.estado, "publicada");
  assert.equal(publicada.revision, 2);
  assert.deepEqual(publicada.puntaje, revision.puntaje);
  assert.equal(evaluacion.entrega_id, entrega.entrega_id);
  await estudiante.getByRole("heading", { name: "Tu devolución" }).waitFor();
  await captura(estudiante, "devolucion");
  const educativo = await api(docente, "/actividades/lr-ensayo/resultados-educativos");
  assert.equal(educativo.intentos.find((fila) => fila.intento_id === alta.intento_id)?.post, "omitido");
  const prueba = {
    entorno: sitio, navegador: navegador.version(), intento: alta.intento_id, version: anterior.version_caso,
    pre: preAcuse, turno, latenciaTurnoMs,
    inferencia: { modelo: traza.invocacion.modelo, digest: traza.invocacion.digest,
      hecho: traza.invocacion.seleccion.hecho_id, fuente: traza.hecho.fuente, linea: traza.hecho.linea,
      duracion_ns: traza.invocacion.duracion_motor_ns, carga_ns: traza.invocacion.carga_modelo_ns },
    presion, pulso, orden, informe, razonamiento, entrega, evaluacionId, post,
    trazaEvaluacion: { operacion: trazaEvaluacion.invocacion.operacion_id, modelo: trazaEvaluacion.invocacion.modelo,
      digest: trazaEvaluacion.invocacion.digest, duracion_ns: trazaEvaluacion.invocacion.duracion_motor_ns,
      citas: criterio.citas.map(({ evidencia }) => evidencia.id) },
    publicacion, puntaje: publicada.puntaje,
  };
  await writeFile(resolve(destino, "recorrido.json"), JSON.stringify(prueba, null, 2) + "\n");
  console.log(JSON.stringify({ intento_id: alta.intento_id, evaluacion_id: evaluacionId, publicada: publicada.revision }));
} finally {
  await navegador.close();
}
