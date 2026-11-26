import assert from "node:assert/strict";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const [destino] = process.argv.slice(2);
assert.ok(destino, "Indica el directorio de evidencia.");
await mkdir(destino, { recursive: true });
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });

try {
  const pagina = await navegador.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
  const boton = (name) => pagina.locator("main").getByRole("button", { name, exact: true });
  await pagina.goto("http://127.0.0.1:8080");
  await pagina.getByLabel("Identidad ficticia").selectOption("docente-demo");
  await boton("Entrar").click();
  await pagina.getByRole("heading", { name: "Supervisión" }).waitFor();
  await boton("Revisar").click();
  await pagina.getByText("Rúbrica V1 · Revisión 2", { exact: true }).waitFor();
  assert.equal(await pagina.getByText("Puntaje 35 de 100.", { exact: true }).count(), 1);
  assert.equal(await pagina.getByRole("heading", { name: "Evidencia del intento" }).count(), 1);
  await pagina.screenshot({ path: resolve(destino, "desglose.png"), fullPage: true });
  const valoracion = pagina.locator("details").filter({ has: pagina.locator("summary").filter({ hasText: /^Valoración ·/ }) });
  await valoracion.locator("summary").click();
  const credito = valoracion.getByLabel("Crédito (%)");
  assert.equal(await credito.inputValue(), "75");
  await credito.fill("50");
  await pagina.getByLabel("Motivo de rectificación").fill("Calibración de navegador de la valoración inicial parcial.");
  const [guardada] = await Promise.all([
    pagina.waitForResponse((r) => r.request().method() === "PATCH" && r.url().includes("/api/evaluaciones/")),
    boton("Guardar revisión").click(),
  ]);
  assert.equal(guardada.status(), 201);
  const rectificacion = await guardada.json();
  assert.equal(rectificacion.anterior, 2);
  assert.equal(rectificacion.revision, 3);
  assert.equal(rectificacion.autor_id, "docente-demo");
  assert.equal(rectificacion.puntaje.total_centesimas, 3000);
  assert.equal(rectificacion.diferencias[0].criterio_id, "valoracion");
  assert.equal(rectificacion.diferencias[0].credito_nuevo, 50);
  await pagina.getByText("Revisión guardada.", { exact: true }).waitFor();
  await pagina.screenshot({ path: resolve(destino, "rectificada.png"), fullPage: true });
  await pagina.reload();
  await pagina.getByRole("heading", { name: "Supervisión" }).waitFor();
  await boton("Revisar").click();
  await pagina.getByText("Rúbrica V1 · Revisión 3", { exact: true }).waitFor();
  assert.equal(await pagina.getByText("Puntaje 30 de 100.", { exact: true }).count(), 1);
  await writeFile(resolve(destino, "navegador.json"), JSON.stringify({
    http: guardada.status(), peticion: guardada.request().postDataJSON(), rectificacion,
    reabierta: 3, total_reabierto: 30,
  }, null, 2) + "\n");
  console.log(JSON.stringify({ revision: rectificacion.revision, total: 30 }));
} finally {
  await navegador.close();
}
