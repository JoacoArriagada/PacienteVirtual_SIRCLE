import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { extname } from "node:path";
import test from "node:test";
import { chromium } from "playwright-core";
import { tipos } from "../scripts/estaticos.mjs";

const origen = "http://127.0.0.1:8487";
const ejecutable = process.env.SIRCLE_CHROMIUM;
const opciones = { skip: ejecutable ? false : "Configura SIRCLE_CHROMIUM para ejecutar en un navegador" };
const intento = {
  intento_id: "intento-prueba", actividad_id: "lr-ensayo",
  version_caso: "lr-ensayo", revision: 1, fase: "inicial", estado: "atencion",
  pre: "omitido", post: "pendiente", rubrica_id: 1,
  introduccion: { paciente: "LR", edad: 30, motivo: "Falta de aire", contexto: "Simulación" },
  ficha: { fuente: "Caso de prueba", fase: "inicial", datos: [] },
  indicaciones: [], ayuda: { origen: "docente", texto: "Ayuda de ensayo" },
};
const turno = {
  turno_id: "turno-prueba", intento_id: intento.intento_id, operacion_id: "operacion-en-curso",
  orden: 1, hablante: "LR", pregunta: "¿Cuándo apareció la falta de aire?",
  respuesta: "Hace dos días.", tipo_respuesta: "hecho",
};
const peticionIntento = "/api/intentos/" + intento.intento_id;

async function enPagina(casos, probar) {
  const navegador = await chromium.launch({ executablePath: ejecutable });
  let sesion = "estudiante";
  try {
    const pagina = await navegador.newPage();
    await pagina.route(origen + "/**", async (route) => {
      const uri = new URL(route.request().url());
      const ruta = uri.pathname;
      const metodo = route.request().method();
      const handler = casos[metodo + " " + ruta];
      if (handler) return handler(route);
      if (ruta === "/api/sesiones") {
        if (metodo === "DELETE") {
          sesion = "";
          return route.fulfill({ status: 204 });
        }
        return sesion
          ? route.fulfill({ json: { rol: sesion } })
          : route.fulfill({ status: 401, json: { mensaje: "Sesión ausente." } });
      }
      if (ruta === "/api/actividades") return route.fulfill({
        json: [{ actividad_id: intento.actividad_id, nombre: "LR", intento_id: null }],
      });
      if (ruta === "/api/intentos" && metodo === "POST") return route.fulfill({ status: 201, json: intento });
      if (ruta.startsWith("/api/")) return route.fulfill({ status: 404, json: { mensaje: "Ruta de prueba desconocida." } });
      const fichero = ruta === "/" ? "/index.html" : ruta;
      const contenido = await readFile(new URL("../dist" + fichero, import.meta.url));
      return route.fulfill({ contentType: tipos.get(extname(fichero)), body: contenido });
    });
    await pagina.goto(origen);
    await probar(pagina);
  } finally {
    await navegador.close();
  }
}

test("un 202 en la entrevista conserva la pregunta y su clave hasta el acuse", opciones, async () => {
  const peticiones = [];
  await enPagina({
    ["POST " + peticionIntento + "/turnos"]: async (route) => {
      const solicitud = route.request();
      peticiones.push({ clave: solicitud.headers()["idempotency-key"], datos: solicitud.postDataJSON() });
      if (peticiones.length === 1) {
        return route.fulfill({ status: 202, headers: { "Retry-After": "1" },
          json: { estado: "en_curso", operacion_id: turno.operacion_id } });
      }
      return route.fulfill({ status: 200, json: turno });
    },
  }, async (pagina) => {
    await pagina.getByRole("button", { name: "Iniciar caso" }).click();
    const pregunta = pagina.getByRole("textbox", { name: "Pregunta al paciente" });
    const enviar = pagina.getByRole("button", { name: "Enviar", exact: true });
    await pregunta.fill(turno.pregunta);
    await enviar.click();
    await pagina.getByText(/solicitud sigue en curso/i).waitFor({ timeout: 2500 });
    assert.equal(await pagina.locator("[data-turno]").count(), 0);
    assert.equal(await pregunta.inputValue(), turno.pregunta);
    await enviar.click();
    await pagina.getByText(turno.respuesta, { exact: true }).waitFor({ timeout: 2500 });
    assert.equal(await pagina.locator("[data-turno]").count(), 1);
    assert.equal(await pregunta.inputValue(), "");
    assert.ok(peticiones[0].clave);
    assert.deepEqual(peticiones, [peticiones[0], peticiones[0]]);
  });
});

test("salir impide que un acuse tardío vuelva a montar una ficha anterior", opciones, async () => {
  const iniciada = Promise.withResolvers();
  const confirmar = Promise.withResolvers();
  await enPagina({
    "POST /api/intentos": async (route) => {
      iniciada.resolve();
      await confirmar.promise;
      return route.fulfill({ status: 201, json: intento });
    },
  }, async (pagina) => {
    await pagina.getByRole("button", { name: "Iniciar caso" }).click();
    await iniciada.promise;
    await pagina.getByRole("button", { name: "Salir", exact: true }).click();
    await pagina.getByRole("heading", { name: "Entrar a SIRCLE" }).waitFor();
    const respuesta = pagina.waitForResponse(origen + "/api/intentos");
    confirmar.resolve();
    await respuesta;
    await pagina.waitForLoadState("networkidle");
    assert.equal(await pagina.getByRole("heading", { name: "Entrar a SIRCLE" }).count(), 1);
    assert.equal(await pagina.getByText(intento.introduccion.motivo, { exact: true }).count(), 0);
  });
});

test("un fallo en la devolución permite volver a consultarla sin recargar", opciones, async () => {
  let consultas = 0;
  await enPagina({
    "GET /api/actividades": (route) => route.fulfill({
      json: [{ actividad_id: intento.actividad_id, nombre: "LR", intento_id: intento.intento_id, estado: "entregado" }],
    }),
    ["GET " + peticionIntento]: (route) => route.fulfill({
      json: { ...intento, estado: "entregado", post: "omitido", entrega: { entrega_id: "entrega-prueba" } },
    }),
    ["GET " + peticionIntento + "/devolucion"]: (route) => {
      consultas++;
      if (consultas === 1) return route.fulfill({ status: 503, json: { mensaje: "Servicio temporalmente indisponible." } });
      return route.fulfill({
        json: {
          estado: "publicada", revision: 2, puntaje: { total_centesimas: 5000 },
          observaciones: [{
            criterio_id: "valoracion", actuacion: "Midió la presión.", expectativa: "Registrar los signos vitales.",
            juicio: "acierto", explicacion: "La lectura respalda la valoración.", mejora: "Registra también el pulso.",
            evidencias: [],
          }],
        },
      });
    },
  }, async (pagina) => {
    await pagina.getByRole("button", { name: "Ver entrega" }).click();
    await pagina.getByText("Servicio temporalmente indisponible.").waitFor();
    await pagina.getByRole("button", { name: "Consultar devolución" }).click({ timeout: 2500 });
    await pagina.getByRole("heading", { name: "Tu devolución" }).waitFor();
    for (const esperado of ["Midió la presión.", "Registrar los signos vitales.", "Acierto",
      "La lectura respalda la valoración.", "Registra también el pulso."]) {
      assert.equal(await pagina.getByText(esperado, { exact: true }).count(), 1);
    }
    assert.equal(consultas, 2);
  });
});

test("la toma de presión y la ficha muestran el origen de la lectura", opciones, async () => {
  const lectura = {
    medicion_id: "presion-prueba", intento_id: intento.intento_id, instrumento: "presion",
    valor: { sistolica: 148, diastolica: 85 }, unidad: "mmHg",
    momento: "2026-11-26T15:00:00.000Z", origen: "Triaje inicial aprobado de LR", en_ficha: false,
  };
  let registrada = false;
  await enPagina({
    ["POST " + peticionIntento + "/mediciones"]: (route) => route.fulfill({ status: 201, json: lectura }),
    ["PUT " + peticionIntento + "/mediciones/" + lectura.medicion_id + "/ficha"]: (route) => {
      registrada = true;
      return route.fulfill({ json: { ...lectura, en_ficha: true } });
    },
    ["GET " + peticionIntento]: (route) => route.fulfill({
      json: { ...intento, ficha: { ...intento.ficha, hallazgos: registrada ? [{ ...lectura, en_ficha: true }] : [] } },
    }),
  }, async (pagina) => {
    await pagina.clock.install();
    await pagina.getByRole("button", { name: "Iniciar caso" }).click();
    await pagina.getByRole("button", { name: "Presión", exact: true }).click();
    await pagina.getByRole("button", { name: "Preparar el brazalete" }).click();
    for (const control of await pagina.getByRole("checkbox").all()) await control.check();
    await pagina.getByRole("button", { name: "Medir presión" }).click();
    await pagina.clock.fastForward(21000);
    await pagina.getByText(lectura.origen, { exact: true }).waitFor();
    await pagina.getByRole("button", { name: "Registrar en ficha" }).click();
    await pagina.getByText("Registrada en ficha", { exact: true }).waitFor();
    await pagina.getByRole("button", { name: "Volver a la atención" }).click();
    await pagina.getByText("Ficha inicial", { exact: true }).click();
    assert.equal(await pagina.getByText("148/85 mmHg · " + lectura.origen + " · " + lectura.momento).count(), 1);
  });
});

test("la práctica sigue disponible cuando el diálogo libre y la evaluación dependen de Ollama", opciones, async () => {
  let disponible = false;
  const aviso = "Revisa los registros y reinicia el servicio de inferencia.";
  await enPagina({
    "GET /api/estado": (route) => route.fulfill({
      json: { base: { disponible: true }, modelo: { disponible, ...(disponible ? {} : { aviso }) } },
    }),
  }, async (pagina) => {
    await pagina.getByRole("button", { name: "Estado de la práctica" }).click({ timeout: 2500 });
    const dialogo = pagina.getByRole("dialog");
    await dialogo.getByText("Práctica · Disponible", { exact: true }).waitFor();
    await dialogo.getByText("Diálogo libre · No disponible", { exact: true }).waitFor();
    await dialogo.getByText("Evaluación · En espera de inferencia", { exact: true }).waitFor();
    await dialogo.getByText(aviso, { exact: true }).waitFor();
    disponible = true;
    await dialogo.getByRole("button", { name: "Actualizar estado" }).click();
    await dialogo.getByText("Diálogo libre · Disponible", { exact: true }).waitFor();
    await dialogo.getByText("Evaluación · Disponible", { exact: true }).waitFor();
    assert.equal(await dialogo.getByText(aviso, { exact: true }).count(), 0);
  });
});
