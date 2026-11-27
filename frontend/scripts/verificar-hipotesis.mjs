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
  await pagina.goto(sitio);
  await pagina.getByLabel("Identidad ficticia").selectOption(identidad);
  await boton(pagina, "Entrar").click();
  const [alta] = await Promise.all([
    pagina.waitForResponse((r) => r.url() === sitio + "/api/intentos" && r.request().method() === "POST"),
    boton(pagina, "Iniciar caso").click(),
  ]);
  assert.equal(alta.status(), 201);
  const { intento_id: id } = await alta.json();
  await boton(pagina, "Omitir encuesta").click();
  await pagina.getByRole("heading", { name: "Atender y conversar" }).waitFor();
  return { id, pagina, ruta: sitio + "/api/intentos/" + id };
}

async function escribir({ pagina, ruta }, metodo, subruta, data, clave, estado) {
  const respuesta = await pagina.request.fetch(ruta + subruta, {
    method: metodo, data, headers: { "Idempotency-Key": clave },
  });
  const cuerpo = await respuesta.json();
  assert.equal(respuesta.status(), estado, JSON.stringify(cuerpo));
  return { http: respuesta.status(), cuerpo };
}

const consulta = async ({ pagina, ruta }, subruta = "") => {
  const respuesta = await pagina.request.get(ruta + subruta);
  assert.equal(respuesta.status(), 200);
  return respuesta.json();
};

const pregunta = { sugerencia_id: "inicio", texto: "¿Desde cuándo comenzó?" };
const ref = (tipo, origen_id) => ({ tipo, origen_id });

try {
  const propio = await abrir("estudiante-lr");
  const ajeno = await abrir("estudiante-sin-asignacion");
  const pagina = propio.pagina;
  const escritura = propio.ruta + "/razonamiento";
  const { cuerpo: turno } = await escribir(propio, "POST", "/turnos", pregunta, "propio-turno", 201);
  const { cuerpo: otroTurno } = await escribir(ajeno, "POST", "/turnos", pregunta, "ajeno-turno", 201);
  const { cuerpo: medicion } = await escribir(propio, "POST", "/mediciones",
    { instrumento: "presion", accion: "toma_completa" }, "propia-medicion", 201);
  const { cuerpo: orden } = await escribir(propio, "POST", "/ordenes",
    { examen: "ecg-inicial", accion: "confirmar" }, "propia-orden", 201);
  const informe = await consulta(propio, "/ordenes/" + orden.orden_id);
  const original = await consulta(propio);

  const editor = pagina.getByRole("tabpanel", { name: "Hipótesis" });
  const principal = editor.getByRole("group", { name: "Hipótesis principal", exact: true });
  const alternativa = editor.getByRole("group", { name: "Alternativa", exact: true });
  const titulo = (grupo) => grupo.getByLabel("Interpretación provisional");
  const fundamento = (grupo) => grupo.getByLabel("Fundamento y límites");
  const seleccionar = async (grupo, referencia) => grupo.locator(`input[value="${referencia}"]`).check();
  async function abrirEditor() {
    await pagina.getByRole("tab", { name: "Hipótesis" }).click();
    await editor.getByRole("button", { name: /^(Fundamentar|Editar) hipótesis$/ }).click();
    await principal.locator(`input[value="${turno.turno_id}"]`).waitFor();
  }
  async function guardar() {
    const [respuesta] = await Promise.all([
      pagina.waitForResponse((r) => r.url() === escritura && r.request().method() === "PUT"),
      boton(pagina, "Guardar hipótesis").click(),
    ]);
    assert.equal(respuesta.status(), 200);
    await editor.getByText("Hipótesis guardadas.", { exact: true }).waitFor();
    return { http: respuesta.status(), cuerpo: await respuesta.json(),
      clave: respuesta.request().headers()["idempotency-key"], entrada: respuesta.request().postDataJSON() };
  }
  async function reabrir() {
    await boton(pagina, "Volver a mis pacientes").click();
    await boton(pagina, "Continuar").click();
    await abrirEditor();
  }

  await abrirEditor();
  await titulo(principal).fill("Hipótesis principal de ensayo");
  const parcial = await guardar();
  assert.deepEqual(parcial.cuerpo, { revision: original.revision + 1, hipotesis: [{
    rol: "principal", titulo: "Hipótesis principal de ensayo", explicacion: "", evidencias: [],
  }] });
  if (destino) await editor.screenshot({ path: resolve(destino, "parcial.png") });

  await reabrir();
  assert.equal(await titulo(principal).inputValue(), parcial.cuerpo.hipotesis[0].titulo);
  assert.equal(await fundamento(principal).inputValue(), "");
  await fundamento(principal).fill("Relaciono el motivo de consulta con el relato y registro la presión medida.");
  await seleccionar(principal, turno.turno_id);
  await seleccionar(principal, medicion.medicion_id);
  await titulo(alternativa).fill("Alternativa de ensayo");
  await fundamento(alternativa).fill("Contrasto la alternativa con el resultado descrito en el ECG inicial.");
  await seleccionar(alternativa, informe.informe_id);
  const confirmada = await guardar();
  assert.equal(confirmada.cuerpo.revision, parcial.cuerpo.revision + 1);
  assert.deepEqual(confirmada.cuerpo.hipotesis.map((item) => item.rol), ["principal", "alternativa"]);
  assert.deepEqual(confirmada.cuerpo.hipotesis[0].evidencias, [ref("turno", turno.turno_id), ref("medicion", medicion.medicion_id)]);
  assert.deepEqual(confirmada.cuerpo.hipotesis[1].evidencias, [ref("informe", informe.informe_id)]);

  const cruces = structuredClone(confirmada.cuerpo.hipotesis);
  cruces[0].evidencias[0] = ref("turno", otroTurno.turno_id);
  const ajena = await escribir(propio, "PUT", "/razonamiento",
    { revision_esperada: confirmada.cuerpo.revision, hipotesis: cruces }, "referencia-ajena", 422);
  const obsoleta = await escribir(propio, "PUT", "/razonamiento",
    { revision_esperada: parcial.cuerpo.revision, hipotesis: [] }, "revision-antigua", 409);
  const reenvio = await escribir(propio, "PUT", "/razonamiento", parcial.entrada, parcial.clave, 200);
  assert.deepEqual(reenvio.cuerpo, parcial.cuerpo);
  const ultima = await consulta(propio);
  assert.deepEqual(ultima.borrador, confirmada.cuerpo);
  assert.equal(ultima.revision, confirmada.cuerpo.revision);

  await reabrir();
  for (const [i, campo] of [principal, alternativa].entries()) {
    const hipot = confirmada.cuerpo.hipotesis[i];
    assert.equal(await titulo(campo).inputValue(), hipot.titulo);
    assert.equal(await fundamento(campo).inputValue(), hipot.explicacion);
    for (const referencia of hipot.evidencias) {
      assert.ok(await campo.locator(`input[value="${referencia.origen_id}"]`).isChecked());
    }
  }
  if (destino) {
    await editor.screenshot({ path: resolve(destino, "reabierta.png") });
    const evidencia = { navegador: navegador.version(), intento: propio.id, intentoAjeno: ajeno.id,
      parcial, confirmada, ajena, obsoleta, reenvio, ultima: ultima.borrador };
    await writeFile(resolve(destino, "hipotesis.json"), JSON.stringify(evidencia, null, 2) + "\n");
  }
  console.log(JSON.stringify({ intento: propio.id, parcial: parcial.cuerpo.revision, completa: confirmada.cuerpo.revision }));
} finally {
  await navegador.close();
}
