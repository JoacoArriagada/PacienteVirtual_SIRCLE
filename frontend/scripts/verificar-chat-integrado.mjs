import assert from "node:assert/strict";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium, request } from "playwright-core";

const raizEvidencia = new URL("../../specs/001-simulacion-clinica/evidencias/h1/", import.meta.url);
const acta = JSON.parse(await readFile(new URL("T020/persistencia.json", raizEvidencia), "utf8")).despues_de_reabrir;
const anterior = JSON.parse(await readFile(new URL("T015/pregunta-libre.json", raizEvidencia), "utf8")).observado.acuse;
const sitio = "http://127.0.0.1:8080";
const ruta = "/api/intentos/" + acta.intento_id;
const pregunta = "¿Desde hace cuánto sientes que te cuesta respirar?";
const evidencia = process.argv[2];
if (evidencia) await mkdir(evidencia, { recursive: true });
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });

try {
  const pagina = await navegador.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
  const trafico = [];
  pagina.on("request", (r) => {
    const url = new URL(r.url());
    if (url.pathname.startsWith("/api/")) trafico.push({ origen: url.origin, ruta: url.pathname, metodo: r.method() });
  });
  const carga = await pagina.goto(sitio);
  assert.equal(carga.status(), 200);
  await pagina.getByLabel("Identidad ficticia").selectOption("estudiante-lr");
  const [sesion] = await Promise.all([
    pagina.waitForResponse((res) => res.url() === sitio + "/api/sesiones" && res.request().method() === "POST"),
    pagina.getByRole("button", { name: "Entrar", exact: true }).click(),
  ]);
  assert.equal(sesion.status(), 201);
  const cookie = (await pagina.context().cookies()).find((valor) => valor.name === "sircle_sesion");
  assert.ok(cookie?.httpOnly && cookie?.sameSite === "Strict");
  await pagina.getByRole("button", { name: "En curso", exact: true }).click();
  const continuar = pagina.getByRole("button", { name: "Continuar", exact: true });
  await continuar.waitFor();
  const captura = async (nombre, elemento = pagina) => {
    if (evidencia) await elemento.screenshot({ path: resolve(evidencia, nombre + ".png") });
  };
  await captura("continuar");
  const [intentada] = await Promise.all([
    pagina.waitForResponse(sitio + ruta),
    continuar.click(),
  ]);
  assert.equal(intentada.status(), 200);
  const previa = await intentada.json();
  assert.equal(previa.intento_id, acta.intento_id);
  assert.equal(previa.version_caso, acta.version_caso);
  assert.deepEqual(previa.turnos.find((turno) => turno.turno_id === anterior.turno_id), anterior);
  const burbuja = pagina.locator('[data-turno="' + anterior.turno_id + '"]');
  await burbuja.getByText(anterior.pregunta, { exact: true }).waitFor();
  await captura("turno-t015", burbuja);

  await pagina.getByLabel("Pregunta al paciente").fill(pregunta);
  const [respuesta] = await Promise.all([
    pagina.waitForResponse(sitio + ruta + "/turnos", { timeout: 200000 }),
    pagina.getByRole("button", { name: "Enviar", exact: true }).click(),
  ]);
  const peticion = respuesta.request();
  const clave = peticion.headers()["idempotency-key"];
  assert.equal(peticion.method(), "POST");
  assert.deepEqual(peticion.postDataJSON(), { texto: pregunta });
  assert.ok(clave && clave !== "t015-parafrasis-inicio");
  assert.equal(respuesta.status(), 201, await respuesta.text());
  const turno = await respuesta.json();
  assert.notEqual(turno.turno_id, anterior.turno_id);
  assert.equal(turno.intento_id, acta.intento_id);
  assert.equal(turno.hablante, "LR");
  assert.match(turno.respuesta, /^Empecé.*hace dos días/);
  const nuevo = pagina.locator('[data-turno="' + turno.turno_id + '"]');
  await nuevo.getByText(turno.respuesta, { exact: true }).waitFor();
  await captura("respuesta-lr", nuevo);

  const revisor = await request.newContext({ baseURL: sitio });
  try {
    const ingreso = await revisor.post("/api/sesiones", { data: { identidad: "docente-demo" } });
    assert.equal(ingreso.status(), 201);
    const respuesta = await revisor.get(ruta + "/turnos/" + turno.turno_id + "/traza");
    assert.equal(respuesta.status(), 200);
    const traza = await respuesta.json();
    assert.equal(traza.turno_id, turno.turno_id);
    assert.equal(traza.operacion_id, turno.operacion_id);
    assert.equal(traza.invocacion.operacion_id, turno.operacion_id);
    assert.equal(traza.invocacion.modelo, "qwen3.5:4b");
    assert.equal(traza.invocacion.seleccion.hecho_id, "lr-inicio");
    assert.equal(traza.hecho.linea, 7);
    assert.equal(traza.hecho.hablante, "LR");

    const [recarga] = await Promise.all([pagina.waitForResponse(sitio + ruta), pagina.reload()]);
    assert.equal(recarga.status(), 200);
    const guardado = await recarga.json();
    for (const esperado of [anterior, turno]) {
      assert.deepEqual(guardado.turnos.find((t) => t.turno_id === esperado.turno_id), esperado);
      await pagina.locator('[data-turno="' + esperado.turno_id + '"]').waitFor();
    }
    const nuevaCookie = (await pagina.context().cookies()).find((valor) => valor.name === "sircle_sesion");
    assert.equal(nuevaCookie.value, cookie.value);
    assert.equal(guardado.turnos.length, previa.turnos.length + 1);
    assert.ok(trafico.every((r) => r.origen === sitio));
    await captura("recarga");
    if (evidencia) {
      const datos = {
        navegador: navegador.version(), sitio, intento_id: guardado.intento_id,
        solicitud: { ruta: peticion.url(), metodo: peticion.method(), cuerpo: peticion.postDataJSON(), clave },
        turno, anterior, traza, recarga: guardado, trafico,
        cookie: { misma: true, HttpOnly: cookie.httpOnly, SameSite: cookie.sameSite },
      };
      await writeFile(resolve(evidencia, "intercambio.json"), JSON.stringify(datos, null, 2) + "\n");
    }
    console.log(JSON.stringify({ intento: guardado.intento_id, t015: anterior.turno_id, nuevo: turno.turno_id, operacion: turno.operacion_id }));
  } finally {
    await revisor.dispose();
  }
} finally {
  await navegador.close();
}
