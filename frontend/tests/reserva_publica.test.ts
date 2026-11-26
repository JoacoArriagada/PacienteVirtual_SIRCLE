import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdir, readdir, readFile, stat, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import test from "node:test";
import { chromium, type Page, type APIResponse } from "playwright-core";

const base = process.env.SIRCLE_RESERVA_DB;
const destino = process.env.SIRCLE_RESERVA_EVIDENCIA;
const sitio = "http://127.0.0.1:8080";
const origen = new URL("../dist/", import.meta.url);
const hash = (buf: Buffer) => createHash("sha256").update(buf).digest("hex");
const boton = (pagina: Page, name: string) => pagina.locator("main").getByRole("button", { name, exact: true });

test("la compilación y la sesión estudiantil mantienen reservado el material de ensayo", { skip: !base }, async () => {
  if (destino) await mkdir(destino, { recursive: true });
  const testigos = JSON.parse(await readFile(new URL("./fixtures/reserva-publica.json", import.meta.url), "utf8")) as Record<string, string>;
  const htmlPrivado = await readFile(new URL("../referencia/semana5/Paciente_Virtual.html", import.meta.url));
  const documentos = await readFile(new URL("../../docs/modulo-01/instrumento-pre-post.md", import.meta.url), "utf8");
  assert.ok(htmlPrivado.includes(testigos.codigo_original));
  assert.ok(documentos.includes(testigos.clave_documentada));
  const salida = resolve(origen.pathname);
  const nombres = (await readdir(salida, { recursive: true })).sort();
  const archivos: { ruta: string; bytes: number; sha256: string }[] = [];

  function comprobar(bytes: Buffer, contexto: string): void {
    assert.ok(!bytes.includes(htmlPrivado), contexto + " · HTML íntegro");
    for (const [tipo, testigo] of Object.entries(testigos)) {
      assert.ok(!bytes.includes(testigo), contexto + " · " + tipo);
    }
    assert.ok(!bytes.includes("taquicardia sinusal"), contexto + " · ECG sin ordenar");
  }

  for (const fichero of nombres) {
    const ruta = join(salida, fichero);
    if (!(await stat(ruta)).isFile()) continue;
    assert.doesNotMatch(fichero, /referencia|ejemplos|rubrica|fixtures|datos|evidencias|\.map$/i);
    const buf = await readFile(ruta);
    comprobar(buf, fichero);
    archivos.push({ ruta: fichero === "index.html" ? "/" : "/" + fichero, bytes: buf.length, sha256: hash(buf) });
  }
  assert.ok(archivos.length > 30);

  const sql = (texto: string) => execFileSync("sqlite3", ["-batch", base!], { input: texto, encoding: "utf8" });
  const literal = (texto: string) => "'" + texto.replaceAll("'", "''") + "'";
  sql(`
    PRAGMA foreign_keys=ON;
    INSERT INTO asignaciones VALUES ('lr-ensayo','estudiante-sin-asignacion','docente-demo');
    CREATE TABLE material_reservado_t038 (tipo TEXT PRIMARY KEY, valor TEXT NOT NULL) STRICT;
    INSERT INTO material_reservado_t038 VALUES
      ('clave', ${literal(testigos.clave)}), ('rubrica', ${literal(testigos.rubrica)}),
      ('borrador_docente', ${literal(testigos.borrador_docente)});
  `);
  const material = sql("SELECT valor FROM material_reservado_t038 ORDER BY tipo;");
  for (const tipo of ["clave", "rubrica", "borrador_docente"]) assert.ok(material.includes(testigos[tipo]));
  assert.ok(sql("SELECT respuesta FROM hechos WHERE id='lr-antecedentes-familiares';").includes(testigos.hecho));

  let fase = "antes_del_pre";
  const examinadas: { fase: string; metodo: string; ruta: string; http: number; bytes: number; cuerpo?: unknown }[] = [];
  const pendientes: Promise<void>[] = [];
  const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });

  function registrar(paso: string, metodo: string, ruta: string, http: number, tipo: string, cuerpo: Buffer): void {
    comprobar(cuerpo, paso + " " + metodo + " " + ruta);
    const datos = tipo.includes("application/json") && cuerpo.length ? JSON.parse(cuerpo.toString("utf8")) : undefined;
    if (datos) assert.doesNotMatch(JSON.stringify(datos), /"(rubrica|ponderaciones|claves|respuesta_correcta|borrador_docente|traza|invocacion|pasaje|revision_fuente_id)"\s*:/);
    examinadas.push({ fase: paso, metodo, ruta, http, bytes: cuerpo.length, cuerpo: datos });
  }

  async function verificarRespuesta(metodo: string, ruta: string, respuesta: APIResponse, estado: number): Promise<Buffer> {
    assert.equal(respuesta.status(), estado, metodo + " " + ruta);
    const cuerpo = await respuesta.body();
    registrar(fase, metodo, ruta, respuesta.status(), respuesta.headers()["content-type"] ?? "", cuerpo);
    return cuerpo;
  }

  async function abrir(pagina: Page, identidad: string): Promise<string> {
    await pagina.goto(sitio);
    await pagina.getByLabel("Identidad ficticia").selectOption(identidad);
    await boton(pagina, "Entrar").click();
    const [alta] = await Promise.all([
      pagina.waitForResponse((res) => res.url() === sitio + "/api/intentos" && res.request().method() === "POST"),
      boton(pagina, "Iniciar caso").click(),
    ]);
    assert.equal(alta.status(), 201);
    return (await alta.json()).intento_id;
  }

  try {
    const dimensiones = { viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 };
    const otra = await navegador.newPage(dimensiones);
    const ajeno = await abrir(otra, "estudiante-sin-asignacion");
    execFileSync("../backend/target/debug/sircle", ["resolver-pre-ensayo", "--db", base!, "--intento", ajeno]);
    const pruebaAjena = await otra.request.put(sitio + "/api/intentos/" + ajeno + "/razonamiento", {
      headers: { "Idempotency-Key": "t038-draft-ajeno" }, data: { revision_esperada: 1,
        hipotesis: [{ rol: "principal", titulo: testigos.borrador_ajeno, explicacion: "", evidencias: [] }] },
    });
    assert.equal(pruebaAjena.status(), 200);
    assert.ok((await pruebaAjena.text()).includes(testigos.borrador_ajeno));

    const pagina = await navegador.newPage(dimensiones);
    pagina.on("response", (respuesta) => {
      const url = new URL(respuesta.url());
      if (url.origin !== sitio || !url.pathname.startsWith("/api/")) return;
      const paso = fase;
      const trabajo = respuesta.body().then((bytes) => {
        registrar(paso, respuesta.request().method(), url.pathname, respuesta.status(),
          respuesta.headers()["content-type"] ?? "", bytes);
      });
      pendientes.push(trabajo);
      void trabajo.catch(() => {});
    });
    const obtener = (ruta: string, estado = 200) => pagina.request.get(sitio + ruta)
      .then((respuesta) => verificarRespuesta("GET", ruta, respuesta, estado));
    const id = await abrir(pagina, "estudiante-lr");
    const intento = "/api/intentos/" + id;
    const ajena = "/api/intentos/" + ajeno;
    const nombresPrivados = [
      "/referencia/semana5/Paciente_Virtual.html",
      "/docs/referencias/html-semana5/Paciente_Virtual.html",
      "/docs/modulo-01/instrumento-pre-post.md",
      "/ejemplos/presion.json",
      "/tests/fixtures/reserva-publica.json",
    ];
    for (const ruta of nombresPrivados) await obtener(ruta, 404);
    for (const archivo of archivos) {
      const descargado = await obtener(archivo.ruta);
      assert.equal(hash(descargado), archivo.sha256, archivo.ruta);
    }
    const pre = JSON.parse((await obtener(intento)).toString("utf8"));
    assert.equal(pre.estado, "pre_pendiente");
    for (const campo of ["introduccion", "ficha", "turnos", "mediciones", "apuntes", "evidencias", "ayuda", "ordenes", "borrador"]) {
      assert.equal(pre[campo], undefined, campo);
    }
    await obtener(intento + "/ordenes/ecg-inicial", 404);
    await obtener(ajena, 403);
    await pagina.getByText("Pre pendiente.", { exact: true }).waitFor();
    comprobar(Buffer.from(await pagina.locator("body").innerText()), fase + " DOM");
    await Promise.all(pendientes);

    fase = "atencion";
    execFileSync("../backend/target/debug/sircle", ["resolver-pre-ensayo", "--db", base!, "--intento", id]);
    await boton(pagina, "Volver a mis pacientes").click();
    await boton(pagina, "Continuar").click();
    await pagina.getByText("Preguntas de ejemplo", { exact: true }).click();
    await boton(pagina, "¿Desde cuándo comenzó?").click();
    const [dijo] = await Promise.all([
      pagina.waitForResponse(sitio + intento + "/turnos"),
      boton(pagina, "Enviar").click(),
    ]);
    assert.equal(dijo.status(), 201);
    const turno = await dijo.json();
    assert.match(turno.respuesta, /hace dos días/i);
    const traza = intento + "/turnos/" + turno.turno_id + "/traza";
    assert.ok(sql("SELECT detalle FROM trazas_turno;").includes(testigos.traza));
    await obtener(traza, 403);
    await obtener(intento + "/ordenes/ecg-inicial", 404);
    const abierto = JSON.parse((await obtener(intento)).toString("utf8"));
    assert.equal(abierto.introduccion.paciente, "LR");
    assert.deepEqual(abierto.turnos, [turno]);
    const apunte = "T038_APUNTE_PROPIO_VISIBLE";
    await pagina.getByLabel("Apunte de la atención").fill(apunte);
    await boton(pagina, "Guardar apunte").click();
    await pagina.getByText("Apunte guardado.", { exact: true }).waitFor();
    assert.equal(await pagina.getByText(apunte, { exact: true }).count(), 1);
    comprobar(Buffer.from(await pagina.locator("body").innerText()), fase + " DOM");
    await obtener(ajena, 403);
    await Promise.all(pendientes);

    fase = "antes_de_publicar";
    assert.match(id, /^[a-f0-9]{32}$/);
    sql("UPDATE intentos SET estado='entregado' WHERE id=" + literal(id) + ";");
    await pagina.reload();
    const entregado = JSON.parse((await obtener(intento)).toString("utf8"));
    assert.equal(entregado.estado, "entregado");
    assert.equal(entregado.apuntes[0].intencion, apunte);
    assert.equal(entregado.turnos[0].turno_id, turno.turno_id);
    comprobar(Buffer.from(await pagina.locator("body").innerText()), fase + " DOM");
    await obtener(traza, 403);
    await obtener(ajena, 403);
    await obtener(intento + "/devolucion", 404);
    await obtener("/api/evaluaciones/ensayo-t038", 404);
    await Promise.all(pendientes);

    if (destino) {
      const inventario = archivos.map((archivo) => [archivo.ruta, archivo.bytes, archivo.sha256].join("\t"));
      await writeFile(resolve(destino, "inventario.tsv"), "ruta\tbytes\tsha256\n" + inventario.join("\n") + "\n");
      await writeFile(resolve(destino, "respuestas.json"), JSON.stringify({ navegador: navegador.version(),
        htmlPrivado: { bytes: htmlPrivado.length, sha256: hash(htmlPrivado) }, testigos: Object.keys(testigos), examinadas }, null, 2) + "\n");
    }
    console.log(JSON.stringify({ archivos: archivos.length, respuestas: examinadas.length,
      fases: [...new Set(examinadas.map((fila) => fila.fase))], coincidencias: 0 }));
  } finally {
    await navegador.close();
  }
});
