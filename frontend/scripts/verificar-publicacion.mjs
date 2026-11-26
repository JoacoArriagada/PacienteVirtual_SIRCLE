import assert from "node:assert/strict";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const [destino, db] = process.argv.slice(2);
assert.ok(destino && db, "Indica evidencia y una copia migrada de la base T062.");
await mkdir(destino, { recursive: true });
const sitio = "http://127.0.0.1:8080";
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });

try {
  async function sesion(identidad) {
    const pagina = await navegador.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
    await pagina.goto(sitio);
    await pagina.getByLabel("Identidad ficticia").selectOption(identidad);
    await pagina.getByRole("button", { name: "Entrar", exact: true }).click();
    return pagina;
  }
  const docente = await sesion("docente-demo");
  await docente.getByRole("button", { name: "Revisar", exact: true }).click();
  await docente.getByText("Rúbrica V1 · Revisión 2", { exact: true }).waitFor();
  const [acuse] = await Promise.all([
    docente.waitForResponse((r) => r.request().method() === "POST" && r.url().endsWith("/publicacion")),
    docente.getByRole("button", { name: "Publicar devolución", exact: true }).click(),
  ]);
  assert.equal(acuse.status(), 201);
  const publicada = await acuse.json();
  assert.equal(publicada.revision, 2);
  await docente.getByText("Revisión publicada 2.", { exact: true }).waitFor();
  await docente.screenshot({ path: resolve(destino, "publicada-docente.png"), fullPage: true });

  const titular = await sesion("estudiante-lr");
  const [pendiente] = await Promise.all([
    titular.waitForResponse((r) => r.url().endsWith("/encuestas/post")),
    titular.getByRole("button", { name: "Ver entrega", exact: true }).click(),
  ]);
  const post = await pendiente.json();
  assert.equal(post.estado, "pendiente");
  assert.equal(post.items.length, 7);
  await titular.getByRole("heading", { name: "Después de entregar" }).waitFor();
  assert.equal(await titular.getByRole("heading", { name: "Tu devolución" }).count(), 0);
  await titular.screenshot({ path: resolve(destino, "post-pendiente.png") });
  const oculto = await titular.request.get(sitio + "/api/intentos/" + post.intento_id + "/devolucion");
  assert.deepEqual(await oculto.json(), { estado: "post_pendiente" });
  const [visible] = await Promise.all([
    titular.waitForResponse((r) => r.url().endsWith("/devolucion")),
    titular.getByRole("button", { name: "Omitir encuesta", exact: true }).click(),
  ]);
  const devolucion = await visible.json();
  assert.equal(devolucion.estado, "publicada");
  assert.equal(devolucion.revision, 2);
  assert.equal(devolucion.puntaje.total_centesimas, 3500);
  await titular.getByRole("heading", { name: "Tu devolución" }).waitFor();
  await titular.getByText("Revisión publicada 2. Puntaje 35 de 100.", { exact: true }).waitFor();
  await titular.screenshot({ path: resolve(destino, "publicada-titular.png"), fullPage: true });
  await writeFile(resolve(destino, "navegador.json"), JSON.stringify({
    publicada, con_post_pendiente: { http: pendiente.status(), estado: "post_pendiente" },
    con_post_omitido: devolucion,
  }, null, 2) + "\n");
  console.log(JSON.stringify({ publicada: publicada.revision, titular: devolucion.estado }));
} finally {
  await navegador.close();
}
