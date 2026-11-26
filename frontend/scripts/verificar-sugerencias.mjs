import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { setTimeout as esperar } from "node:timers/promises";
import { chromium, request } from "playwright-core";

const [destino, base] = process.argv.slice(2);
assert.ok(base, "Indica la ruta de la base temporal.");
if (destino) await mkdir(destino, { recursive: true });
const frontend = "http://127.0.0.1:4173";
const backend = "http://127.0.0.1:8081";
const ollama = "http://127.0.0.1:11434";
const contenedor = "sircle-ollama-h1";
const literal = "¿Desde cuándo comenzó?";
const editada = "¿Conoces los antecedentes médicos de tu familia biológica?";
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
try {
  const pagina = await navegador.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
  await pagina.route(frontend + "/api/**", async (ruta) => {
    const respuesta = await ruta.fetch({ url: ruta.request().url().replace(frontend, backend), timeout: 195000 });
    await ruta.fulfill({ response: respuesta });
  });
  await pagina.goto(frontend + "/aplicacion");
  await pagina.getByLabel("Identidad ficticia").selectOption("estudiante-lr");
  await pagina.getByRole("button", { name: "Entrar", exact: true }).click();
  const [alta] = await Promise.all([
    pagina.waitForResponse((r) => r.url() === frontend + "/api/intentos" && r.request().method() === "POST"),
    pagina.getByRole("button", { name: "Iniciar caso", exact: true }).click(),
  ]);
  assert.equal(alta.status(), 201);
  const { intento_id: id } = await alta.json();
  execFileSync("../backend/target/debug/sircle", ["resolver-pre-ensayo", "--db", base, "--intento", id]);
  await pagina.getByRole("button", { name: "Volver a mis pacientes", exact: true }).click();
  await pagina.getByRole("button", { name: "Continuar", exact: true }).click();
  await pagina.getByText("Preguntas de ejemplo", { exact: true }).click();
  const ejemplo = pagina.getByRole("button", { name: literal, exact: true });
  const entrada = pagina.getByLabel("Pregunta al paciente");
  const enviar = pagina.getByRole("button", { name: "Enviar", exact: true });
  const ruta = "/api/intentos/" + id + "/turnos";

  async function registrar() {
    const [respuesta] = await Promise.all([
      pagina.waitForResponse(frontend + ruta, { timeout: 198000 }),
      enviar.click(),
    ]);
    assert.equal(respuesta.status(), 201, await respuesta.text());
    return {
      http: respuesta.status(),
      peticion: respuesta.request().postDataJSON(),
      clave: respuesta.request().headers()["idempotency-key"],
      acuse: await respuesta.json(),
    };
  }

  await assert.rejects(fetch(ollama + "/api/tags"), (error) => error.cause?.code === "ECONNREFUSED");
  await ejemplo.click();
  assert.equal(await entrada.inputValue(), literal);
  if (destino) await pagina.screenshot({ path: resolve(destino, "texto-intacto.png"), fullPage: true });
  const directa = await registrar();
  assert.deepEqual(directa.peticion, { texto: literal, sugerencia_id: "inicio" });
  assert.match(directa.acuse.respuesta, /hace dos días/);
  await pagina.locator("#chat-log").getByText(directa.acuse.respuesta, { exact: true }).waitFor();

  execFileSync("docker", ["start", contenedor]);
  let digest;
  for (let intento = 0; !digest && intento < 40; intento++) {
    try {
      const r = await fetch(ollama + "/api/tags");
      if (r.ok) {
        const tags = await r.json();
        digest = tags.models.find((modelo) => modelo.name === "qwen3.5:4b")?.digest;
      }
    } catch {}
    if (!digest) await esperar(500);
  }
  assert.ok(digest, "No se recuperó el motor con el modelo local.");
  await ejemplo.click();
  await entrada.fill(editada);
  assert.equal(await ejemplo.getAttribute("aria-pressed"), "false");
  if (destino) await pagina.screenshot({ path: resolve(destino, "texto-editado.png"), fullPage: true });
  const libre = await registrar();
  assert.deepEqual(libre.peticion, { texto: editada });
  assert.equal(libre.acuse.hablante, "LR");
  assert.match(libre.acuse.respuesta, /fui adoptada/);
  await pagina.locator("#chat-log").getByText(libre.acuse.respuesta, { exact: true }).waitFor();

  const docente = await request.newContext({ baseURL: backend });
  const login = await docente.post("/api/sesiones", { data: { identidad: "docente-demo" } });
  assert.equal(login.status(), 201);
  const trazas = [];
  for (const turno of [directa, libre]) {
    const res = await docente.get(ruta + "/" + turno.acuse.turno_id + "/traza");
    assert.equal(res.status(), 200);
    trazas.push(await res.json());
  }
  await docente.dispose();
  assert.equal(trazas[0].origen, "ficha");
  assert.equal(trazas[0].invocacion, null);
  assert.equal(trazas[1].origen, "ollama");
  assert.equal(trazas[1].invocacion.digest, digest);
  assert.equal(trazas[1].invocacion.operacion_id, libre.acuse.operacion_id);
  assert.deepEqual(trazas[1].invocacion.seleccion, { hecho_id: "lr-antecedentes-familiares" });
  if (destino) {
    await pagina.screenshot({ path: resolve(destino, "respuesta-de-lr.png"), fullPage: true });
    const informe = { navegador: navegador.version(), id, directa, libre, trazas };
    await writeFile(resolve(destino, "rutas.json"), JSON.stringify(informe, null, 2) + "\n");
  }
  console.log(JSON.stringify({ id, literal: directa.acuse.turno_id, libre: libre.acuse.turno_id, digest }));
} finally {
  await navegador.close();
}
