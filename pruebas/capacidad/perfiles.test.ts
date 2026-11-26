import assert from "node:assert/strict";
import { execFile } from "node:child_process";
import { createHash } from "node:crypto";
import { appendFileSync, mkdtempSync, readFileSync, rmSync, rmdirSync, writeFileSync } from "node:fs";
import { createServer, type IncomingMessage, type ServerResponse } from "node:http";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { DatabaseSync } from "node:sqlite";
import test from "node:test";
import { setTimeout as esperar } from "node:timers/promises";
import { fileURLToPath } from "node:url";
import { promisify } from "node:util";

test("los perfiles concilian la ráfaga y observan las sesiones separadas", async () => {
  const carpeta = mkdtempSync(join(tmpdir(), "sircle-capacidad-"));
  const base = join(carpeta, "ensayo.sqlite");
  const salida = join(carpeta, "informe.json");
  const salidaSesiones = join(carpeta, "sesiones.json");
  const log = join(carpeta, "backend.log");
  const db = new DatabaseSync(base);
  db.exec(`
    CREATE TABLE rubricas_activas_ensayo (actividad_id TEXT);
    INSERT INTO rubricas_activas_ensayo VALUES ('lr-ensayo');
    CREATE TABLE identidades (id TEXT, rol TEXT);
    CREATE TABLE asignaciones (actividad_id TEXT, estudiante_id TEXT, docente_id TEXT);
  `);
  db.close();
  writeFileSync(log, "Arranque de prueba\n");

  const intentos = new Map<string, { identidad: string; orden: number; lecturas: number;
    solicitudes: string[]; turno?: { turno_id: string; intento_id: string; operacion_id: string; pregunta: string };
    medicion?: { intento_id: string; medicion_id: string }; entrega?: { intento_id: string; entrega_id: string } }>();
  let mezcla = false;
  let inferenciasActivas = 0;
  const simultaneas = Promise.withResolvers<void>();
  async function esperarInferencias() {
    inferenciasActivas++;
    if (inferenciasActivas === 2) simultaneas.resolve();
    await simultaneas.promise;
    await esperar(20);
  }
  const simulador = createServer(async (peticion, res) => {
    const ruta = peticion.url ?? "";
    if (ruta === "/api/estado") return json(res, 200, {
      base: { disponible: true },
      modelo: { nombre: "qwen-prueba", digest_esperado: "huella-de-prueba", disponible: true },
    });
    if (ruta === "/api/tags") return json(res, 200, {
      models: [{ name: "qwen-prueba", digest: "huella-de-prueba", details: { quantization_level: "ensayo" } }],
    });
    if (ruta === "/api/ps") return json(res, 200, { models: [{ name: "qwen-prueba", size_vram: 1024 }] });
    if (ruta === "/api/sesiones") {
      const { identidad } = await cuerpo(peticion);
      res.setHeader("Set-Cookie", "ensayo=" + identidad);
      return json(res, 201, {});
    }
    const identidad = peticion.headers.cookie?.split("=")[1];
    if (!identidad) return json(res, 401, {});
    if (ruta === "/api/intentos") {
      const orden = intentos.size + 1;
      const id = "intento-" + orden;
      intentos.set(id, { identidad, orden, lecturas: 0, solicitudes: [] });
      return json(res, 201, { intento_id: id });
    }
    const [, , , id, accion] = ruta.split("/");
    const intento = intentos.get(id);
    if (!intento || intento.identidad !== identidad) return json(res, 403, {});
    if (accion === "encuestas") return json(res, 200, {});
    if (accion === "mediciones") {
      intento.medicion = { intento_id: id, medicion_id: "medicion-" + id };
      return json(res, 201, intento.medicion);
    }
    if (accion === "entrega") {
      const { revision_esperada } = await cuerpo(peticion);
      if (revision_esperada !== 7) return json(res, 409, {});
      const entrega = { intento_id: id, entrega_id: "entrega-" + id };
      intento.entrega = entrega;
      const inicio = Date.now();
      registrar("SIRCLE_EVALUACION_TOMA", { entrega_id: entrega.entrega_id, inicio_unix_ms: inicio });
      await esperarInferencias();
      registrar("SIRCLE_INFERENCIA_EVALUACION", {
        entrega_id: entrega.entrega_id, operacion_id: "evaluacion-1", inicio_unix_ms: inicio, fin_unix_ms: Date.now(),
      });
      return json(res, 201, entrega);
    }
    if (accion !== "turnos") {
      intento.lecturas++;
      if (!mezcla && intento.orden === 3 && intento.lecturas > 1) return json(res, 503, {});
      return json(res, 200, { intento_id: id, estudiante_id: identidad, revision: 7,
        turnos: intento.turno ? [intento.turno] : [],
        mediciones: intento.medicion ? [intento.medicion] : [], entrega: intento.entrega });
    }
    const { texto } = await cuerpo(peticion);
    const clave = String(peticion.headers["idempotency-key"]);
    intento.solicitudes.push(clave);
    const huella = createHash("sha256").update("POST turnos\n" + JSON.stringify(texto)).digest("hex");
    const hash = createHash("sha256");
    for (const parte of [identidad, id, clave, huella]) {
      const longitud = Buffer.alloc(8);
      longitud.writeBigUInt64BE(BigInt(Buffer.byteLength(parte)));
      hash.update(longitud).update(parte);
    }
    const operacion = hash.digest("hex").slice(0, 32);
    const turno = { intento_id: id, operacion_id: operacion, turno_id: "turno-" + id, pregunta: texto };
    if (mezcla) {
      intento.turno = turno;
      const inicio = Date.now();
      registrar("SIRCLE_TURNO_RECIBIDO", { operacion_id: operacion, recibido_unix_ms: inicio });
      registrar("SIRCLE_TRABAJADOR_OPERACION", { operacion_id: operacion, inicio_unix_ms: inicio });
      await esperarInferencias();
      registrar("SIRCLE_INFERENCIA", { operacion_id: operacion, inicio_unix_ms: inicio, fin_unix_ms: Date.now() });
      return json(res, 201, turno);
    }
    if (intento.orden === 1) {
      if (intento.solicitudes.length === 1) return json(res, 503, {});
      if (intento.solicitudes.length === 2) {
        res.setHeader("Retry-After", "1");
        return json(res, 202, { estado: "en_curso", operacion_id: operacion });
      }
      intento.turno = turno;
      const recepcion = Date.now() - 280;
      registrar("SIRCLE_TURNO_RECIBIDO", { operacion_id: operacion, recibido_unix_ms: recepcion });
      registrar("SIRCLE_TRABAJADOR_OPERACION", { operacion_id: operacion, inicio_unix_ms: recepcion + 20 });
      registrar("SIRCLE_INFERENCIA", { operacion_id: operacion, inicio_unix_ms: recepcion + 30, fin_unix_ms: recepcion + 250 });
      registrar("SIRCLE_TURNO_CONFIRMADO", { operacion_id: operacion, confirmado_unix_ms: recepcion + 260 });
      return json(res, 200, turno);
    }
    if (intento.orden === 2) return json(res, 201, turno);
    if (intento.orden === 5) {
      res.setHeader("Retry-After", "20");
      return json(res, 202, { estado: "en_curso", operacion_id: operacion });
    }
    return json(res, 422, {});
  });
  function registrar(evento: string, datos: object) {
    appendFileSync(log, `backend-1  | ${new Date().toISOString()} ${evento} ${JSON.stringify(datos)}\n`);
  }
  try {
    await new Promise<void>((resolve, reject) => {
      simulador.once("error", reject);
      simulador.listen(0, "127.0.0.1", resolve);
    });
    const direccion = simulador.address();
    assert.ok(direccion && typeof direccion !== "string");
    const url = "http://127.0.0.1:" + direccion.port;
    await promisify(execFile)(process.execPath, [
      fileURLToPath(new URL("preguntas_simultaneas.ts", import.meta.url)),
      "--base", base, "--salida", salida, "--url", url, "--ollama-url", url,
      "--log", log, "--usuarios", "5", "--espera-http", "4",
    ], { windowsHide: true });
    const informe = JSON.parse(readFileSync(salida, "utf8"));
    assert.equal(informe.caracter, "exploratorio");
    assert.equal(informe.resultados.dialogo.total, 5);
    assert.equal(informe.resultados.dialogo.completadas, 1);
    assert.equal(informe.conciliacion.cruces, 0);
    assert.equal(informe.conciliacion.historiales_inaccesibles, 1);
    assert.equal(informe.conciliacion.estados_turno.no_conciliado, 1);
    assert.equal(informe.conciliacion.estados_turno.fallido, 1);
    assert.equal(informe.conciliacion.estados_turno.en_curso, 1);
    assert.equal(informe.conciliacion.turnos_con_acuse_en_curso, 2);
    assert.equal(informe.conciliacion.perdidos, 1);
    assert.equal(informe.conciliacion.recuperados, 1);
    assert.equal(informe.conciliacion.esperas_p95_ms, 20);
    assert.equal(informe.conciliacion.pico_inferencias_observadas, 1);
    assert.equal(informe.modelo.vram_pico_bytes, 1024);
    const solicitudes = intentos.get("intento-1")?.solicitudes ?? [];
    assert.equal(solicitudes.length, 3);
    assert.equal(new Set(solicitudes).size, 1);
    const recibidas = informe.muestras.filter((muestra: { intento: string }) => muestra.intento === "intento-1");
    assert.deepEqual(recibidas.map((muestra: { http: number }) => muestra.http), [503, 202, 200]);
    assert.ok(recibidas[2].enviado_ms - recibidas[1].enviado_ms >= 1000);
    assert.ok(informe.turnos.find((turno: { estado: string }) => turno.estado === "recuperado").latencia_hasta_acuse_ms >= 1000);
    assert.equal(intentos.get("intento-4")?.solicitudes.length, 1);

    intentos.clear();
    mezcla = true;
    await promisify(execFile)(process.execPath, [
      fileURLToPath(new URL("sesiones_abiertas.ts", import.meta.url)),
      "--base", base, "--salida", salidaSesiones, "--url", url, "--log", log, "--usuarios", "4", "--espera-http", "4",
    ], { windowsHide: true });
    const sesiones = JSON.parse(readFileSync(salidaSesiones, "utf8"));
    assert.equal(sesiones.conciliacion.historiales_propios, 4);
    assert.equal(sesiones.conciliacion.accesos_ajenos_denegados, 4);
    assert.equal(sesiones.conciliacion.cruces, 0);
    assert.equal(sesiones.conciliacion.escrituras_ausentes, 0);
    assert.equal(sesiones.resultados.entregar.completadas, 1);
    assert.equal(sesiones.telemetria.entregas_con_toma_observada, 1);
    assert.equal(sesiones.telemetria.intervalos_evaluacion, 1);
    assert.equal(sesiones.conciliacion.pico_inferencias_entrevista_observadas, 1);
    assert.equal(sesiones.conciliacion.pico_inferencias_observadas, 2);
  } finally {
    await new Promise<void>((resolve) => simulador.close(() => resolve()));
    for (const ruta of [base, salida, salidaSesiones, log]) rmSync(ruta, { force: true });
    rmdirSync(carpeta);
  }
});

function json(res: ServerResponse, codigo: number, datos: object) {
  res.writeHead(codigo, { "Content-Type": "application/json" }).end(JSON.stringify(datos));
}

async function cuerpo(peticion: IncomingMessage) {
  let texto = "";
  for await (const bloque of peticion) texto += bloque;
  return JSON.parse(texto);
}
