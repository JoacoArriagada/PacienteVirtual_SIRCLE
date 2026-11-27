import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const [destino, db] = process.argv.slice(2);
assert.ok(destino && db, "Indica el directorio de evidencia y una copia de la base de T055.");
await mkdir(destino, { recursive: true });
const sitio = "http://127.0.0.1:8080";
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });

try {
  const pagina = await navegador.newPage({ viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 });
  const boton = (name) => pagina.locator("main").getByRole("button", { name, exact: true });
  await pagina.goto(sitio);
  await pagina.getByLabel("Identidad ficticia").selectOption("estudiante-lr");
  await boton("Entrar").click();
  const [vista] = await Promise.all([
    pagina.waitForResponse((r) => /\/api\/intentos\/[0-9a-f]+$/.test(r.url())),
    boton("Continuar").click(),
  ]);
  const inicial = await vista.json();
  execFileSync("../backend/target/debug/sircle", ["resolver-pre-ensayo", "--db", db, "--intento", inicial.intento_id]);
  await pagina.reload();
  await boton("Revisar entrega").click();
  const titulo = pagina.getByRole("heading", { name: "Revisar antes de entregar" });
  await titulo.waitFor();
  assert.ok(await titulo.evaluate((h) => document.activeElement === h));
  const revision = pagina.getByRole("region", { name: "Revisar antes de entregar" });
  assert.equal(await revision.getByText("Sin hipótesis guardada", { exact: true }).count(), 1);
  await boton("Revisar Razonamiento").click();
  await pagina.getByRole("dialog", { name: "Razonamiento" }).waitFor();
  await pagina.getByRole("button", { name: "Cerrar ventana" }).click();
  await pagina.screenshot({ path: resolve(destino, "revisar.png") });
  await boton("Seguir editando").click();
  await boton("Revisar entrega").click();

  const acuses = [];
  await pagina.route("**/api/intentos/*/entrega", async (ruta) => {
    const recibida = await ruta.fetch();
    acuses.push({
      http: recibida.status(), cuerpo: await recibida.json(), clave: ruta.request().headers()["idempotency-key"],
      peticion: ruta.request().postDataJSON(),
    });
    if (acuses.length === 1) await ruta.abort();
    else await ruta.fulfill({ response: recibida });
  });
  const confirmar = boton("Confirmar entrega");
  await confirmar.click();
  await pagina.waitForFunction(() => {
    const aviso = document.querySelector("#titulo-entrega")?.closest("section")?.querySelector("[data-aviso]");
    return Boolean(aviso?.textContent);
  });
  await confirmar.click();
  await pagina.getByRole("heading", { name: "Entrega confirmada" }).waitFor();
  await pagina.getByRole("heading", { name: "Después de entregar" }).waitFor();
  assert.equal(await boton("Revisar entrega").count(), 0);
  assert.deepEqual(acuses.map(({ http }) => http), [201, 200]);
  assert.deepEqual(acuses[0].cuerpo, acuses[1].cuerpo);
  assert.equal(acuses[0].clave, acuses[1].clave);
  assert.equal(acuses[0].cuerpo.version_rubrica, "1");
  await pagina.screenshot({ path: resolve(destino, "confirmada.png") });
  await boton("Volver a mis pacientes").click();
  await pagina.getByRole("button", { name: "Finalizados", exact: true }).click();
  await boton("Ver entrega").click();
  await pagina.getByRole("heading", { name: "Entrega confirmada" }).waitFor();
  const reabierta = await pagina.locator("[data-entrega-id]").getAttribute("data-entrega-id");
  assert.equal(reabierta, acuses[0].cuerpo.entrega_id);
  await writeFile(resolve(destino, "navegador.json"), JSON.stringify({ acuses, reabierta }, null, 2) + "\n");
  console.log(JSON.stringify({ acuses: acuses.map((acuse) => acuse.http), entrega: reabierta }));
} finally {
  await navegador.close();
}
