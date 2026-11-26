import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const [destino, pre = "respondido", post = "omitido", modo] = process.argv.slice(2);
const proyecto = process.env.SIRCLE_E22_COMPOSE_PROJECT;
assert.ok(destino && [pre, post].every((estado) => ["respondido", "omitido"].includes(estado)),
  "Indica el destino de la evidencia y las decisiones respondido u omitido");
assert.ok(modo === undefined || (modo === "--solo-caida" && proyecto),
  "Usa --solo-caida con un proyecto Compose de ensayo que tenga la inferencia detenida");
await mkdir(destino, { recursive: true });
const recuperar = proyecto && modo !== "--solo-caida";
const puerto = proyecto ? compose("port", "frontend", "8080").trim() : "127.0.0.1:8080";
assert.match(puerto, /^127\.0\.0\.1:\d+$/, "El frontend del ensayo debe estar publicado en loopback");
const sitio = "http://" + puerto;
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
let detener = false;

function compose(...argumentos) {
  return execFileSync("docker", ["compose", "--project-name", proyecto, ...argumentos], {
    cwd: new URL("../../", import.meta.url), encoding: "utf8", stdio: ["ignore", "pipe", "inherit"],
  });
}

async function abrir(identidad) {
  const pagina = await navegador.newPage({ viewport: { width: 1280, height: 900 } });
  await pagina.goto(sitio);
  await pagina.getByLabel("Identidad ficticia").selectOption(identidad);
  await pagina.getByRole("button", { name: "Entrar", exact: true }).click();
  return pagina;
}

async function api(pagina, ruta, opciones = {}, codigo = 200) {
  const respuesta = await pagina.request.fetch(sitio + "/api" + ruta, opciones);
  assert.equal(respuesta.status(), codigo, await respuesta.text());
  return respuesta.json();
}

async function resolver(pagina, momento, estado) {
  const formulario = pagina.locator("form.encuesta");
  await formulario.waitFor();
  assert.equal(await formulario.locator("fieldset").count(), momento === "pre" ? 5 : 7);
  await pagina.screenshot({ path: resolve(destino, momento + ".png"), fullPage: true });
  if (estado === "respondido") {
    for (const campo of await formulario.locator("fieldset").all()) {
      await campo.getByRole("radio").first().check();
    }
  }
  const [respuesta] = await Promise.all([
    pagina.waitForResponse((r) => r.url().endsWith("/encuestas/" + momento) && r.request().method() === "PUT"),
    formulario.getByRole("button", { name: estado === "omitido" ? "Omitir encuesta" : "Enviar respuestas", exact: true }).click(),
  ]);
  assert.equal(respuesta.status(), 200, await respuesta.text());
  const acuse = await respuesta.json();
  assert.equal(acuse.estado, estado);
  return acuse;
}

function escribir(metodo, clave, data) {
  return { method: metodo, data, headers: { "Idempotency-Key": clave } };
}

try {
  const estudiante = await abrir("estudiante-lr");
  await estudiante.getByRole("button", { name: "Iniciar caso", exact: true }).click();
  await estudiante.getByRole("heading", { name: "Antes de la práctica" }).waitFor();
  const id = new URL(estudiante.url()).hash.split("/")[1];
  const intento = "/intentos/" + id;
  assert.equal((await api(estudiante, intento)).introduccion, undefined);
  assert.equal(await estudiante.getByText("Falta de aire", { exact: true }).count(), 0);
  const preAcuse = await resolver(estudiante, "pre", pre);
  await estudiante.getByRole("heading", { name: "Falta de aire", exact: true }).waitFor();
  await estudiante.reload();
  await estudiante.getByRole("heading", { name: "Falta de aire", exact: true }).waitFor();
  assert.equal((await api(estudiante, intento)).pre, pre);

  const medicionesIniciales = (await api(estudiante, intento)).mediciones;
  assert.deepEqual(medicionesIniciales, []);
  const turno = await api(estudiante, intento + "/turnos",
    escribir("POST", "e22-turno", { texto: "¿Desde cuándo comenzó?", sugerencia_id: "inicio" }), 201);
  let antesDeCaida = null;
  if (recuperar) {
    antesDeCaida = await api(estudiante, "/estado");
    assert.equal(antesDeCaida.modelo.disponible, true, "Espera la verificación inicial de Qwen");
    detener = true;
    compose("stop", "inferencia");
  }
  const arranque = await api(estudiante, "/estado");
  assert.equal(arranque.base.disponible, true);
  if (proyecto) assert.equal(arranque.modelo.disponible, false);
  const medicion = await api(estudiante, intento + "/mediciones",
    escribir("POST", "e22-presion", { instrumento: "presion", accion: "toma_completa" }), 201);
  const guardado = await api(estudiante, intento + "/razonamiento",
    escribir("PUT", "e22-hipotesis", { revision_esperada: medicion.revision, hipotesis: [{
      rol: "principal", titulo: "Hipótesis de ensayo",
      explicacion: "La hipótesis cita la medición de este intento.",
      evidencias: [{ tipo: "medicion", origen_id: medicion.medicion_id }],
    }] }));
  const peticionEntrega = escribir("POST", "e22-entrega", { revision_esperada: guardado.revision });
  const entrega = await api(estudiante, intento + "/entrega", peticionEntrega, 201);
  assert.deepEqual(await api(estudiante, intento + "/entrega", peticionEntrega), entrega);
  await estudiante.reload();
  await estudiante.getByRole("heading", { name: "Entrega confirmada" }).waitFor();
  assert.deepEqual(await api(estudiante, intento + "/devolucion"), { estado: "post_pendiente" });
  const postAcuse = await resolver(estudiante, "post", post);
  const guardadoFinal = await api(estudiante, intento);
  assert.equal(guardadoFinal.post, post);
  assert.deepEqual(guardadoFinal.mediciones.map((fila) => fila.medicion_id), [medicion.medicion_id]);
  assert.deepEqual(guardadoFinal.entrega, entrega);
  const docente = await abrir("docente-demo");
  const rutaGrupo = "/actividades/lr-ensayo";
  await docente.getByRole("heading", { name: "Supervisión" }).waitFor();
  const buscarEntrega = async () => (await api(docente, rutaGrupo + "/entregas"))
    .filter((fila) => fila.intento_id === id);
  let antes = await buscarEntrega();
  if (!arranque.modelo.disponible) {
    const limite = Date.now() + 30_000;
    while (antes[0]?.estado === "en_curso" && Date.now() < limite) {
      await new Promise((resolver) => setTimeout(resolver, 250));
      antes = await buscarEntrega();
    }
    assert.equal(antes[0]?.estado, "pendiente");
    assert.equal(antes[0]?.evaluacion_id, null);
  }
  assert.equal(antes.length, 1);
  const educativo = await api(docente, rutaGrupo + "/resultados-educativos");
  assert.equal(educativo.intentos.find((fila) => fila.intento_id === id).post, post);
  await docente.getByText("Resultados educativos de la actividad", { exact: true }).click();
  await docente.getByRole("heading", { name: "Razonamiento por forma y orden" }).waitFor();
  await docente.screenshot({ path: resolve(destino, "informe.png"), fullPage: true });

  let despues = null;
  if (recuperar) {
    compose("start", "inferencia");
    detener = false;
    const limite = Date.now() + 12 * 60_000;
    while (Date.now() < limite) {
      const propia = await buscarEntrega();
      assert.equal(propia.length, 1);
      if (propia[0].estado === "terminado") {
        despues = propia[0];
        break;
      }
      await new Promise((resolver) => setTimeout(resolver, 2000));
    }
    assert.ok(despues?.evaluacion_id, "No terminó la evaluación tras recuperar Ollama");
    assert.equal(despues.entrega_id, entrega.entrega_id);
    assert.deepEqual(await api(estudiante, intento + "/entrega", peticionEntrega), entrega);
    const reanudado = await api(estudiante, intento);
    assert.deepEqual(reanudado.turnos.map((fila) => fila.turno_id), [turno.turno_id]);
    assert.deepEqual(reanudado.mediciones.map((fila) => fila.medicion_id), [medicion.medicion_id]);
    assert.deepEqual(reanudado.entrega, entrega);
    assert.deepEqual(await buscarEntrega(), [despues]);
    assert.equal((await api(estudiante, "/estado")).modelo.disponible, true);
  }
  const evidencia = { entorno: proyecto ? "Compose" : "procesos locales", sitio, antesDeCaida, arranque,
    pre: preAcuse, post: postAcuse, turno, medicion, entrega, antes, despues };
  const nombre = recuperar ? "e22-compose.json" : proyecto ? "durante-caida-compose.json" : "encuestas.json";
  await writeFile(resolve(destino, nombre), JSON.stringify(evidencia, null, 2) + "\n");
  console.log(JSON.stringify({ intento: id, pre, post, evaluacion: despues?.evaluacion_id }));
} finally {
  try {
    if (detener) compose("start", "inferencia");
  } finally {
    await navegador.close();
  }
}
