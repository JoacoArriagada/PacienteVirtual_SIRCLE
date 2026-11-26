import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import { chromium } from "playwright-core";

const [destino, base] = process.argv.slice(2);
assert.ok(base);
if (destino) await mkdir(destino, { recursive: true });
const sitio = "http://127.0.0.1:8080";
const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM, ignoreDefaultArgs: ["--mute-audio"] });
const opciones = { viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 };
const boton = (pagina, name) => pagina.locator("main").getByRole("button", { name, exact: true });
function pcmOnda(muestras, frecuencia, ganancia) {
  const onda = Buffer.alloc(44 + 2 * muestras.length);
  onda.write("RIFF", 0);
  onda.writeUInt32LE(onda.length - 8, 4);
  onda.write("WAVEfmt ", 8);
  onda.writeUInt32LE(16, 16);
  onda.writeUInt16LE(1, 20);
  onda.writeUInt16LE(1, 22);
  onda.writeUInt32LE(frecuencia, 24);
  onda.writeUInt32LE(frecuencia * 2, 28);
  onda.writeUInt16LE(2, 32);
  onda.writeUInt16LE(16, 34);
  onda.write("data", 36);
  onda.writeUInt32LE(muestras.length * 2, 40);
  for (const [indice, muestra] of muestras.entries()) {
    onda.writeInt16LE(Math.round(muestra * ganancia * 32767), 44 + 2 * indice);
  }
  return onda;
}

try {
  const original = await navegador.newPage(opciones);
  const nueva = await navegador.newPage(opciones);
  for (const pagina of [original, nueva]) {
    await pagina.clock.install();
    await pagina.addInitScript(() => {
      const conectar = AudioNode.prototype.connect;
      const iniciar = AudioBufferSourceNode.prototype.start;
      const detener = AudioBufferSourceNode.prototype.stop;
      const conexiones = new WeakMap();
      window.pruebaAudio = { inicios: [], paradas: 0 };
      AudioNode.prototype.connect = function (...argumentos) {
        conexiones.set(this, argumentos[0]);
        return conectar.apply(this, argumentos);
      };
      AudioBufferSourceNode.prototype.start = function (...argumentos) {
        const resultado = iniciar.apply(this, argumentos);
        const ganancia = conexiones.get(this);
        window.pruebaAudio.inicios.push({
          frecuencia: this.buffer.sampleRate,
          muestras: Array.from(this.buffer.getChannelData(0)),
          bucle: this.loop,
          contexto: this.context.state,
          ganancia: ganancia.gain.value,
          salida: conexiones.get(ganancia) === this.context.destination,
        });
        return resultado;
      };
      AudioBufferSourceNode.prototype.stop = function (...argumentos) {
        const resultado = detener.apply(this, argumentos);
        window.pruebaAudio.paradas++;
        return resultado;
      };
    });
  }
  await original.goto(new URL("../referencia/semana5/Paciente_Virtual.html", import.meta.url).href);
  await boton(original, "Iniciar caso").first().click();
  await boton(original, "Comenzar atención").click();
  await nueva.goto(sitio);
  await nueva.getByLabel("Identidad ficticia").selectOption("estudiante-lr");
  await boton(nueva, "Entrar").click();
  const [alta] = await Promise.all([
    nueva.waitForResponse((r) => r.url() === sitio + "/api/intentos" && r.request().method() === "POST"),
    boton(nueva, "Iniciar caso").click(),
  ]);
  assert.equal(alta.status(), 201);
  const { intento_id: intento } = await alta.json();
  execFileSync("../backend/target/debug/sircle", ["resolver-pre-ensayo", "--db", base, "--intento", intento]);
  await boton(nueva, "Volver a mis pacientes").click();
  await boton(nueva, "Continuar").click();
  const ruta = "/api/intentos/" + intento;
  const casos = [
    { modo: "Espalda", zona: "postLeft", etiqueta: "Base izquierda", habilitada: true, duracion: 2.4 },
    { modo: "Corazón", zona: "mitral", etiqueta: "Foco mitral", habilitada: false, duracion: 0.82 },
  ];
  const registros = [];
  for (const caso of casos) {
    const estados = {};
    const ondas = {};
    for (const [nombre, pagina] of [["original", original], ["extraccion", nueva]]) {
      const herramienta = nombre === "original"
        ? pagina.locator('main button[data-do="instrument"][data-value="auscultacion"]')
        : boton(pagina, "Auscultación");
      await herramienta.click();
      const repetir = nombre === "original" && registros.length > 0;
      await boton(pagina, repetir ? "Repetir toma" : "Preparar el fonendoscopio").click();
      await boton(pagina, caso.modo).click();
      const iniciar = boton(pagina, "Auscultar");
      for (const casilla of await pagina.locator("main").getByRole("checkbox").all()) await casilla.check();
      assert.equal(await iniciar.isDisabled(), true);
      await boton(pagina, caso.etiqueta).click();
      assert.equal(await iniciar.isEnabled(), true);
      const escena = pagina.locator("#device-art");
      assert.equal(await escena.locator(".exam-target").count(), 1);
      assert.equal(await boton(pagina, caso.etiqueta).getAttribute("aria-pressed"), "true");
      const foto = await escena.screenshot({ animations: "disabled" });
      const caja = await escena.boundingBox();
      estados[nombre] = {
        ancho: caja.width, alto: caja.height, huella: createHash("sha256").update(foto).digest("hex"),
      };
      if (destino) await pagina.screenshot({ path: resolve(destino, caso.zona + "-" + nombre + ".png"), animations: "disabled" });
      await pagina.evaluate(() => {
        window.pruebaAudio.inicios.length = 0;
        window.pruebaAudio.paradas = 0;
        let semilla = 2026;
        Math.random = () => {
          semilla = (Math.imul(1664525, semilla) + 1013904223) >>> 0;
          return semilla / 4294967296;
        };
      });
      await iniciar.click();
      await boton(pagina, "Escuchar audio de práctica").click();
      await boton(pagina, "Detener audio").waitFor();
      const { inicios } = await pagina.evaluate(() => window.pruebaAudio);
      assert.equal(inicios.length, 1);
      const { muestras, frecuencia, bucle, contexto, ganancia, salida } = inicios[0];
      assert.equal(contexto, "running");
      assert.equal(salida, true);
      assert.equal(bucle, true);
      assert.equal(muestras.length, Math.floor(frecuencia * caso.duracion));
      const rms = Math.sqrt(muestras.reduce((suma, valor) => suma + (valor * ganancia) ** 2, 0) / muestras.length);
      assert.ok(rms > 0);
      const onda = pcmOnda(muestras, frecuencia, ganancia);
      ondas[nombre] = { rms, frecuencia, ganancia, segundos: muestras.length / frecuencia,
        huella: createHash("sha256").update(onda).digest("hex") };
      if (destino && nombre === "extraccion") {
        await writeFile(resolve(destino, caso.zona + ".wav"), onda);
        await pagina.screenshot({ path: resolve(destino, caso.zona + "-reproduccion.png"), animations: "disabled" });
      }
      if (nombre === "original") {
        await boton(pagina, "Detener audio").click();
        assert.equal(await pagina.evaluate(() => window.pruebaAudio.paradas), 1);
      }
    }
    assert.deepEqual(estados.original, estados.extraccion);
    assert.deepEqual(ondas.original, ondas.extraccion);
    await original.clock.fastForward(9000);
    await original.locator("main").getByRole("heading", { name: "Resultado disponible", exact: true }).waitFor();
    if (destino) await original.screenshot({ path: resolve(destino, caso.zona + "-resultado-original.png"), animations: "disabled" });
    await boton(original, "Volver a la atención").click();
    const [respuesta] = await Promise.all([
      nueva.waitForResponse((r) => r.url() === sitio + ruta + "/exploraciones" && r.request().method() === "POST"),
      nueva.clock.fastForward(9000),
    ]);
    const acuse = await respuesta.json();
    assert.equal(respuesta.status(), 201, JSON.stringify(acuse));
    assert.equal(acuse.intento_id, intento);
    assert.equal(acuse.instrumento, "auscultacion");
    assert.equal(await nueva.evaluate(() => window.pruebaAudio.paradas), 1);
    assert.equal(acuse.zona, caso.zona);
    assert.equal(acuse.etiqueta, caso.etiqueta);
    assert.ok(acuse.momento.endsWith("Z"));
    if (caso.habilitada) {
      assert.equal(acuse.estado, "habilitado");
      assert.ok(acuse.hallazgo.hallazgo_id);
      assert.notEqual(acuse.exploracion_id, acuse.hallazgo.hallazgo_id);
      await nueva.getByText(acuse.hallazgo.texto, { exact: true }).waitFor();
    } else {
      assert.equal(acuse.estado, "pendiente");
      assert.equal(acuse.hallazgo, undefined);
      await nueva.getByText("Hallazgo pendiente", { exact: true }).waitFor();
    }
    if (destino) await nueva.screenshot({ path: resolve(destino, caso.zona + "-resultado.png"), animations: "disabled" });
    registros.push({ zona: caso.zona, estados, ondas, acuse });
    await boton(nueva, "Volver a la atención").click();
  }
  const vista = await nueva.evaluate(async (ruta) => (await fetch(ruta)).json(), ruta);
  assert.deepEqual(vista.exploraciones, registros.map((fila) => fila.acuse));
  assert.equal(vista.exploraciones.filter((e) => e.hallazgo?.hallazgo_id).length, 1);
  if (destino) {
    const datos = { navegador: navegador.version(), intento, registros };
    await writeFile(resolve(destino, "acciones.json"), JSON.stringify(datos, null, 2) + "\n");
  }
  console.log(JSON.stringify({ intento, acciones: vista.exploraciones.length, hallazgos: vista.exploraciones.filter((e) => e.hallazgo).length }));
} finally {
  await navegador.close();
}
