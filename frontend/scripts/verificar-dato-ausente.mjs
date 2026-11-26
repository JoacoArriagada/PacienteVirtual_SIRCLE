import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium, request } from "playwright-core";

const [destino, base] = process.argv.slice(2);
assert.ok(base, "Indica la ruta absoluta de una base nueva de ensayo.");
if (destino) await mkdir(destino, { recursive: true });
const sitio = "http://127.0.0.1:8080";
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
const revisor = await request.newContext({ baseURL: sitio });
const texto = "¿Cuál es su grupo sanguíneo?";

try {
  const pagina = await navegador.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
  const boton = (name) => pagina.getByRole("button", { name, exact: true });
  await pagina.goto(sitio);
  await pagina.getByLabel("Identidad ficticia").selectOption("estudiante-lr");
  await boton("Entrar").click();
  const [alta] = await Promise.all([
    pagina.waitForResponse((r) => r.url() === sitio + "/api/intentos" && r.request().method() === "POST"),
    boton("Iniciar caso").click(),
  ]);
  assert.equal(alta.status(), 201);
  const { intento_id: id } = await alta.json();
  const ruta = "/api/intentos/" + id;
  execFileSync("../backend/target/debug/sircle", ["resolver-pre-ensayo", "--db", base, "--intento", id]);
  await boton("Volver a mis pacientes").click();
  await boton("Continuar").click();
  const antes = await (await pagina.request.get(sitio + ruta)).json();
  assert.deepEqual(antes.turnos, []);
  await pagina.getByLabel("Pregunta al paciente").fill(texto);
  const [respuesta] = await Promise.all([
    pagina.waitForResponse(sitio + ruta + "/turnos", { timeout: 200000 }),
    boton("Enviar").click(),
  ]);
  const turno = await respuesta.json();
  assert.equal(respuesta.status(), 201, JSON.stringify(turno));
  assert.equal(turno.hablante, "Sistema");
  assert.equal(turno.tipo_respuesta, "dato_ausente");
  assert.equal(turno.respuesta, "El grupo sanguíneo no está disponible en este escenario.");
  const aviso = pagina.locator(`[data-turno="${turno.turno_id}"][data-tipo-respuesta="dato_ausente"] .bubble.system`);
  await aviso.getByText("Sistema", { exact: true }).waitFor();
  await aviso.getByText(turno.respuesta, { exact: true }).waitFor();
  assert.equal(await pagina.locator(`[data-turno="${turno.turno_id}"] .bubble.patient`).count(), 0);
  const despues = await (await pagina.request.get(sitio + ruta)).json();
  assert.deepEqual(despues.turnos, [turno]);
  assert.deepEqual(despues.ficha, antes.ficha);
  assert.deepEqual(despues.evidencias, antes.evidencias);
  const inicio = await revisor.post("/api/sesiones", { data: { identidad: "docente-demo" } });
  assert.equal(inicio.status(), 201);
  const detalle = await revisor.get(ruta + "/turnos/" + turno.turno_id + "/traza");
  assert.equal(detalle.status(), 200);
  const traza = await detalle.json();
  assert.equal(traza.motivo, "dato_no_registrado");
  assert.equal(traza.hecho, null);
  assert.equal(traza.invocacion.seleccion.hecho_id, "sin_hecho");
  const motor = await (await fetch("http://127.0.0.1:11434/api/ps")).json();
  const qwen = motor.models.find((item) => item.name === traza.invocacion.modelo);
  assert.ok(qwen && qwen.size_vram === 0 && qwen.digest === traza.invocacion.digest);
  if (destino) {
    await pagina.locator(".notebook").screenshot({ path: resolve(destino, "aviso.png") });
    const evidencia = { navegador: navegador.version(), intento: id, peticion: { texto }, http: respuesta.status(),
      turno, traza, motor: qwen, antes: { ficha: antes.ficha, evidencias: antes.evidencias, turnos: antes.turnos },
      despues: { ficha: despues.ficha, evidencias: despues.evidencias, turnos: despues.turnos } };
    await writeFile(resolve(destino, "dato-ausente.json"), JSON.stringify(evidencia, null, 2) + "\n");
  }
  console.log(JSON.stringify({ intento: id, turno: turno.turno_id, tipo: turno.tipo_respuesta, hallazgosNuevos: 0 }));
} finally {
  await revisor.dispose();
  await navegador.close();
}
