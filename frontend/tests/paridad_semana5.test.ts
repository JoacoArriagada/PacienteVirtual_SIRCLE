import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { resolve } from "node:path";
import test from "node:test";
import { chromium, type Locator, type Page } from "playwright-core";

const base = process.env.SIRCLE_PARIDAD_DB;
const destino = process.env.SIRCLE_PARIDAD_EVIDENCIA;
const sitio = "http://127.0.0.1:" + (process.env.SIRCLE_PUERTO_FRONTEND ?? "8080");
const dimensiones = { viewport: { width: 1280, height: 900 }, deviceScaleFactor: 1 };
const fuente = new URL("../../docs/referencias/html-semana5/Paciente_Virtual.html", import.meta.url);
const copia = new URL("../referencia/semana5/Paciente_Virtual.html", import.meta.url);
const boton = (pagina: Page, name: string) => pagina.locator("main").getByRole("button", { name, exact: true });
const huella = (archivo: Buffer) => createHash("sha256").update(archivo).digest("hex");

interface Captura {
  ancho: number;
  alto: number;
  radios: string[];
  huella: string;
}

test("Semana 5 permite comparar selección, chat, brazo, presión e histórico por teclado", { skip: !base }, async () => {
  if (destino) await mkdir(destino, { recursive: true });
  const html = await readFile(fuente);
  assert.equal(huella(html), huella(await readFile(copia)));
  const navegador = await chromium.launch({ executablePath: process.env.SIRCLE_CHROMIUM });
  const evidencias: Record<string, Record<string, Captura>> = { original: {}, extraccion: {} };
  const escenas: Record<string, Map<string, Buffer>> = { original: new Map(), extraccion: new Map() };
  const teclado: Record<string, unknown[]> = { original: [], extraccion: [] };

  async function capturar(pagina: Page, interfaz: string, estado: string, region: Locator): Promise<void> {
    await pagina.evaluate(() => document.fonts.ready);
    const png = await region.screenshot({ animations: "disabled" });
    const caja = await region.boundingBox();
    assert.ok(caja);
    const radios = await region.evaluate((elemento) => {
      const estilo = getComputedStyle(elemento);
      return [estilo.borderTopLeftRadius, estilo.borderTopRightRadius, estilo.borderBottomRightRadius, estilo.borderBottomLeftRadius];
    });
    evidencias[interfaz][estado] = { ancho: caja.width, alto: caja.height, radios, huella: huella(png) };
    if (["brazo", "preparacion", "colocado"].includes(estado)) escenas[interfaz].set(estado, png);
    if (destino) {
      await writeFile(resolve(destino, interfaz + "-" + estado + "-detalle.png"), png);
      await pagina.screenshot({ path: resolve(destino, interfaz + "-" + estado + ".png"), animations: "disabled" });
    }
  }

  async function porTeclado(pagina: Page, interfaz: string, destinoFoco: Locator, nombre: string): Promise<void> {
    const enfocado = () => destinoFoco.evaluate((el) => el === document.activeElement);
    let pasos = 0;
    while (!(await enfocado()) && pasos < 36) {
      await pagina.keyboard.press("Tab");
      pasos++;
    }
    assert.ok(await enfocado(), `${interfaz}: ${nombre}`);
    const foco = await destinoFoco.evaluate((el) => {
      const css = getComputedStyle(el);
      return { selector: el.id || el.getAttribute("name") || el.textContent?.trim(),
        visible: el.matches(":focus-visible"), estilo: css.outlineStyle,
        ancho: css.outlineWidth, contraste: css.outlineColor, desplazamiento: css.outlineOffset };
    });
    assert.equal(foco.visible, true, `${interfaz}: ${nombre}`);
    assert.equal(foco.estilo, "solid");
    assert.ok(Number.parseInt(foco.ancho) >= 2);
    teclado[interfaz].push({ accion: nombre, pasosTab: pasos, foco });
  }

  try {
    const original = await navegador.newPage(dimensiones);
    const extraccion = await navegador.newPage(dimensiones);
    original.setDefaultTimeout(10000);
    extraccion.setDefaultTimeout(10000);
    await original.clock.install();
    await extraccion.clock.install();
    await original.goto(fuente.href);
    await original.locator("#patient-search").fill("LR");
    await extraccion.goto(sitio);
    await extraccion.getByLabel("Identidad ficticia").selectOption("estudiante-lr");
    await boton(extraccion, "Entrar").click();
    const rutas: Record<string, string> = {};
    const lecturas: Record<string, number[]> = {};
    const estiloChat: Record<string, unknown> = {};
    const estadosVentana: Record<string, unknown> = {};
    const rutaIntencion = sitio + "/api/intentos";
    for (const [interfaz, pagina] of [["original", original], ["extraccion", extraccion]] as const) {
      const carta = pagina.locator("#catalog-cards .patient-card");
      await carta.waitFor();
      assert.equal(await carta.count(), 1);
      estadosVentana[interfaz] = await pagina.evaluate(() => ({
        ancho: innerWidth, alto: innerHeight, dpr: devicePixelRatio, zoom: visualViewport?.scale,
      }));
      await capturar(pagina, interfaz, "seleccion", carta);
      const comenzar = boton(pagina, "Iniciar caso");
      if (interfaz === "original") {
        await comenzar.click();
        await boton(pagina, "Comenzar atención").click();
      } else {
        const [alta] = await Promise.all([
          pagina.waitForResponse((r) => r.url() === rutaIntencion && r.request().method() === "POST"),
          comenzar.click(),
        ]);
        assert.equal(alta.status(), 201);
        const { intento_id } = await alta.json();
        rutas.intento = rutaIntencion + "/" + intento_id;
        await boton(pagina, "Omitir encuesta").click();
      }

      if ((await pagina.locator("#questions").getAttribute("open")) === null) {
        await pagina.getByText("Preguntas de ejemplo", { exact: true }).click();
      }
      await boton(pagina, "¿Desde cuándo comenzó?").click();
      await boton(pagina, "Enviar").click();
      const chat = pagina.locator("#chat-log");
      const burbuja = chat.locator(".bubble.student");
      await burbuja.getByText("¿Desde cuándo comenzó?", { exact: true }).waitFor();
      await chat.getByText(/hace dos días/i).last().waitFor();
      estiloChat[interfaz] = await chat.evaluate((registro) => {
        const escolar = registro.querySelector(".bubble.student")!;
        const leyenda = escolar.querySelector("p")!;
        const contenedor = getComputedStyle(registro);
        const estilo = getComputedStyle(escolar);
        return { fuente: estilo.fontFamily, fuenteMensaje: getComputedStyle(leyenda).fontSize,
          alineacion: estilo.alignSelf, fondo: estilo.backgroundColor, radio: estilo.borderRadius,
          espacio: contenedor.gap, alto: contenedor.height };
      });
      await capturar(pagina, interfaz, "chat", pagina.locator(".notebook"));
      await boton(pagina, "Ver brazo").click();
      await pagina.getByRole("heading", { name: "Vista del brazo · LR", exact: true }).waitFor();
      await capturar(pagina, interfaz, "brazo", pagina.locator(".arm-image"));
      await pagina.keyboard.press("Escape");
      await pagina.getByRole("dialog").waitFor({ state: "hidden" });

      await boton(pagina, "Presión").click();
      const preparar = boton(pagina, "Preparar el brazalete");
      await porTeclado(pagina, interfaz, preparar, "Preparar brazalete");
      await pagina.keyboard.press("Enter");
      const escena = pagina.locator("#device-art");
      const medir = boton(pagina, "Medir presión");
      const controles = pagina.getByRole("region", { name: "Controles del instrumento" });
      assert.equal(await medir.isDisabled(), true);
      await capturar(pagina, interfaz, "preparacion", escena);
      const pasos = controles.getByRole("checkbox");
      for (const [i, nombre] of ["Seleccionar brazalete", "Colocar brazalete", "Confirmar posición"].entries()) {
        await porTeclado(pagina, interfaz, pasos.nth(i), nombre);
        await pagina.keyboard.press("Space");
        assert.equal(await pasos.nth(i).isChecked(), true);
        if (i === 1) {
          assert.equal(await escena.getAttribute("data-positioned"), "true");
          assert.equal(await medir.isDisabled(), true);
          await capturar(pagina, interfaz, "colocado", escena);
        }
      }
      assert.equal(await medir.isEnabled(), true);
      await porTeclado(pagina, interfaz, medir, "Medir presión");
      await pagina.keyboard.press("Enter");
      await pagina.clock.runFor(3000);
      await capturar(pagina, interfaz, "toma", controles.locator(".live-monitor"));
      await pagina.clock.fastForward(21000);
      await controles.getByRole("heading", { name: "Resultado disponible" }).waitFor();
      lecturas[interfaz] = await controles.locator(".metrics dd strong").allTextContents().then((valores) => valores.map(Number));
      const registrar = boton(pagina, "Registrar en ficha");
      await porTeclado(pagina, interfaz, registrar, "Leer resultado y registrar");
      await capturar(pagina, interfaz, "resultado", controles.locator(".metrics"));
      await pagina.keyboard.press("Enter");
      await controles.getByText(/registrad[oa] en ficha/i).first().waitFor();
      await porTeclado(pagina, interfaz, boton(pagina, "Histórico"), "Abrir histórico");
      await pagina.keyboard.press("Enter");
      const historia = pagina.locator(".procedure-view section.panel").filter({
        has: pagina.getByRole("button", { name: "Tabla", exact: true }),
      });
      await historia.getByRole("img", { name: /Presión arterial por toma/ }).waitFor();
      await capturar(pagina, interfaz, "historico-grafica", historia);
      await porTeclado(pagina, interfaz, boton(pagina, "Tabla"), "Consultar tabla");
      await pagina.keyboard.press("Space");
      await historia.getByRole("table", { name: "Tomas del intento" }).waitFor();
      await capturar(pagina, interfaz, "historico-tabla", historia);
    }
    assert.deepEqual(estadosVentana.original, estadosVentana.extraccion);
    assert.deepEqual(estadosVentana.original, { ancho: 1280, alto: 900, dpr: 1, zoom: 1 });
    assert.deepEqual(estiloChat.original, estiloChat.extraccion);
    const rasterizacion: Record<string, Awaited<ReturnType<typeof compararPixeles>>> = {};
    for (const [estado, foto] of escenas.original) {
      const original = evidencias.original[estado];
      const extraida = evidencias.extraccion[estado];
      assert.deepEqual([original.ancho, original.alto], [extraida.ancho, extraida.alto], estado);
      assert.deepEqual(original.radios, extraida.radios, estado);
      const radios = original.radios.map((radio) => {
        assert.match(radio, /^\d+(?:\.\d+)?px$/);
        return Number.parseFloat(radio);
      });
      const diferencia = await compararPixeles(extraccion, foto, escenas.extraccion.get(estado)!, radios);
      assert.ok(diferencia.deltaInterior <= 1 && diferencia.deltaContorno <= 8,
        JSON.stringify({ estado, diferencia }));
      rasterizacion[estado] = diferencia;
    }
    assert.deepEqual(lecturas.original, [128, 78, 112]);
    assert.deepEqual(lecturas.extraccion, [148, 85]);
    const respuesta = await extraccion.request.post(rutas.intento + "/mediciones", {
      headers: { "Idempotency-Key": "t037-pulso" },
      data: { instrumento: "pulso", accion: "toma_completa" },
    });
    assert.equal(respuesta.status(), 201);
    const pulso = await respuesta.json();
    assert.equal(pulso.valor.frecuencia, 102);
    const observado = { navegador: navegador.version(), entorno: estadosVentana, referenciaSha256: huella(html),
      estiloChat, estados: evidencias, rasterizacion, teclado, lecturas, pulsoExtraido: pulso };
    if (destino) await writeFile(resolve(destino, "comparacion.json"), JSON.stringify(observado, null, 2) + "\n");
    console.log(JSON.stringify({ navegador: navegador.version(), interfaz: estadosVentana,
      pares: Object.keys(evidencias.original).length, hitosTeclado: teclado.original.length, lecturas }));
  } finally {
    await navegador.close();
  }
});

function compararPixeles(pagina: Page, original: Buffer, extraccion: Buffer, radios: number[]) {
  return pagina.evaluate(async ({ capturas, radios }) => {
    const pixeles = await Promise.all(capturas.map(async (base64) => {
      const bytes = Uint8Array.from(atob(base64), (caracter) => caracter.charCodeAt(0));
      const imagen = await createImageBitmap(new Blob([bytes], { type: "image/png" }));
      const canvas = new OffscreenCanvas(imagen.width, imagen.height);
      const ctx = canvas.getContext("2d")!;
      ctx.drawImage(imagen, 0, 0);
      return ctx.getImageData(0, 0, canvas.width, canvas.height);
    }));
    const [a, b] = pixeles;
    if (a.width !== b.width || a.height !== b.height) throw new Error("Las capturas tienen dimensiones distintas");
    let diferentes = 0;
    let deltaInterior = 0;
    let deltaContorno = 0;
    let deltaMaximo = 0;
    for (let i = 0; i < a.data.length; i += 4) {
      const delta = Math.max(...[0, 1, 2, 3].map((canal) => Math.abs(a.data[i + canal] - b.data[i + canal])));
      if (delta === 0) continue;
      diferentes++;
      deltaMaximo = Math.max(deltaMaximo, delta);
      const x = (i / 4) % a.width + 0.5;
      const y = Math.floor(i / 4 / a.width) + 0.5;
      const distancias = [[x, y], [a.width - x, y], [a.width - x, a.height - y], [x, a.height - y]];
      // El margen de un píxel y medio contempla el redondeo del recorte y el antialias del arco.
      const enContorno = radios.some((radio, esquina) => {
        const [dx, dy] = distancias[esquina];
        return radio > 0 && dx <= radio && dy <= radio
          && Math.abs(Math.hypot(radio - dx, radio - dy) - radio) <= 1.5;
      });
      if (enContorno) deltaContorno = Math.max(deltaContorno, delta);
      else deltaInterior = Math.max(deltaInterior, delta);
    }
    return { diferentes, deltaInterior, deltaContorno, deltaMaximo, fraccion: diferentes / (a.width * a.height) };
  }, { capturas: [original, extraccion].map((imagen) => imagen.toString("base64")), radios });
}
