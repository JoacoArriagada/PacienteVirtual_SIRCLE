import assert from "node:assert/strict";
import { createHash, randomUUID } from "node:crypto";
import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { arch, cpus, freemem, platform, totalmem } from "node:os";
import { DatabaseSync } from "node:sqlite";
import { setTimeout as esperar } from "node:timers/promises";
import { parseArgs } from "node:util";

type Perfil = "rr01" | "sesiones" | "rafaga";
type Accion = "abrir" | "guardar" | "dialogo" | "entregar";
type Datos = {
  base?: { disponible: boolean };
  modelo?: { nombre: string; digest_esperado: string; contexto_entrevista: number; disponible: boolean };
  estudiante_id?: string;
  intento_id?: string;
  revision?: number;
  estado?: string;
  operacion_id?: string;
  turno_id?: string;
  medicion_id?: string;
  entrega_id?: string;
  turnos?: { turno_id: string; intento_id: string; operacion_id: string; pregunta: string }[];
  mediciones?: { intento_id: string; medicion_id: string }[];
  entrega?: { intento_id: string; entrega_id: string };
};
type Respuesta = {
  http: number; contenido: Datos | null; cookie: string | null; reintentar_ms?: number;
  error?: "tiempo_agotado" | "transporte";
};
type Cliente = { identidad: string; cookie: string; intento: string; revision: number };
type Muestra = {
  identidad: string; intento: string; accion: Accion; enviado_ms: number; duracion_ms: number;
  http: number; confirmada: boolean; cruce: boolean; en_curso: boolean; recuperacion: boolean;
  operacion?: string; registro?: string; reintentar_ms?: number; error?: Respuesta["error"];
};
type TurnoEsperado = { cliente: Cliente; texto: string; clave: string; operacion: string; solicitudes: Muestra[] };

export async function ejecutar(perfil: Perfil): Promise<void> {
  const { values } = parseArgs({ options: {
    base: { type: "string" }, url: { type: "string", default: "http://127.0.0.1:8080" },
    salida: { type: "string" }, log: { type: "string" }, "ollama-url": { type: "string" },
    usuarios: { type: "string" }, segundos: { type: "string", default: "900" },
    "espera-http": { type: "string", default: "600" },
  } });
  assert.ok(values.base && values.salida, "Indica --base de ensayo y --salida para guardar el informe");
  assert.ok(statSync(values.base).isFile(), "La base de ensayo debe existir");
  assert.ok(!existsSync(values.salida), "Usa una ruta de informe nueva");
  const origen = new URL(values.url);
  assert.ok(origen.protocol === "http:" && ["127.0.0.1", "localhost", "backend"].includes(origen.hostname),
    "Indica la API local de ensayo");
  const previstos = perfil === "rr01" ? 20 : 100;
  const usuarios = Number(values.usuarios ?? previstos);
  const segundos = Number(values.segundos);
  const esperaMs = Number(values["espera-http"]) * 1000;
  assert.ok(Number.isInteger(usuarios) && usuarios >= 1 && usuarios <= previstos, "Cantidad de usuarios fuera del perfil");
  assert.ok(Number.isSafeInteger(segundos) && segundos > 0
    && Number.isSafeInteger(esperaMs) && esperaMs >= 1000 && esperaMs <= 2_147_483_647,
  "Usa duraciones positivas en segundos");
  const peticion = (ruta: string, metodo = "GET", contenido?: object, cookie?: string, clave?: string, limite = esperaMs) =>
    consultar(origen, ruta, limite, metodo, contenido, cookie, clave);
  const estado = await peticion("/estado");
  assert.equal(estado.http, 200, "La API local debe responder al estado antes del ensayo");
  assert.equal(estado.contenido?.base?.disponible, true, "La base de ensayo debe estar disponible en la API");
  const logInicio = values.log ? statSync(values.log).size : 0;
  const clientes: Cliente[] = [];
  for (const identidad of asignar(values.base, usuarios)) {
    const sesion = await peticion("/sesiones", "POST", { identidad });
    assert.equal(sesion.http, 201, "La API debe usar la misma base de ensayo");
    assert.ok(sesion.cookie);
    const alta = await peticion("/intentos", "POST", { actividad_id: "lr-ensayo" }, sesion.cookie, randomUUID());
    assert.equal(alta.http, 201);
    const intento = alta.contenido?.intento_id;
    assert.ok(intento);
    const pre = await peticion(`/intentos/${intento}/encuestas/pre`, "PUT", { estado: "omitido" }, sesion.cookie, randomUUID());
    assert.equal(pre.http, 200);
    const vista = await peticion("/intentos/" + intento, "GET", undefined, sesion.cookie);
    const revision = vista.contenido?.revision;
    assert.ok(vista.http === 200 && vista.contenido?.estudiante_id === identidad
      && vista.contenido.intento_id === intento && typeof revision === "number" && Number.isInteger(revision),
    "La sesión de ensayo debe recuperar su propio intento y revisión");
    clientes.push({ identidad, cookie: sesion.cookie, intento, revision });
  }
  const preguntas = [
    "¿Cuándo empezaste a notar que te faltaba el aire?",
    "¿Conoces tus antecedentes familiares?",
  ];
  const muestras: Muestra[] = [];
  const esperadas = new Map<string, TurnoEsperado>();
  const usar = async (cliente: Cliente, accion: Accion, indice: number, reenvio?: TurnoEsperado, limite = esperaMs) => {
    const ruta = "/intentos/" + cliente.intento;
    const clave = reenvio?.clave ?? randomUUID();
    const texto = reenvio?.texto ?? preguntas[indice % preguntas.length];
    const operacion = accion === "dialogo" ? idTurno(cliente, clave, texto) : undefined;
    const turno: TurnoEsperado | undefined = reenvio
      ?? (operacion ? { cliente, texto, clave, operacion, solicitudes: [] } : undefined);
    if (turno && !reenvio) esperadas.set(turno.operacion, turno);
    const enviado = Date.now();
    const inicio = performance.now();
    let respuesta: Respuesta;
    switch (accion) {
      case "abrir": respuesta = await peticion(ruta, "GET", undefined, cliente.cookie); break;
      case "guardar": respuesta = await peticion(ruta + "/mediciones", "POST",
        { instrumento: "presion", accion: "toma_completa" }, cliente.cookie, clave); break;
      case "dialogo": respuesta = await peticion(ruta + "/turnos", "POST", { texto }, cliente.cookie, clave, limite); break;
      case "entregar": respuesta = await peticion(ruta + "/entrega", "POST",
        { revision_esperada: cliente.revision }, cliente.cookie, clave); break;
    }
    const acuse = respuesta.http === 200 || respuesta.http === 201;
    const { intento_id, estudiante_id, operacion_id, turno_id, medicion_id, entrega_id, estado } = respuesta.contenido ?? {};
    const cruce = (acuse || respuesta.http === 202)
      && ((intento_id !== undefined && intento_id !== cliente.intento)
      || (estudiante_id !== undefined && estudiante_id !== cliente.identidad)
      || (operacion_id !== undefined && operacion_id !== operacion));
    const registro = { abrir: intento_id, guardar: medicion_id, entregar: entrega_id, dialogo: turno_id }[accion];
    const confirmada = acuse && !cruce && intento_id === cliente.intento && !!registro
      && (accion !== "dialogo" || operacion_id === operacion)
      && (accion !== "abrir" || estudiante_id === cliente.identidad);
    const muestra: Muestra = {
      identidad: cliente.identidad, intento: cliente.intento, accion, enviado_ms: enviado,
      duracion_ms: Math.round(performance.now() - inicio), http: respuesta.http, confirmada, cruce,
      en_curso: respuesta.http === 202 && !cruce && operacion !== undefined && operacion_id === operacion && estado === "en_curso",
      reintentar_ms: respuesta.reintentar_ms, error: respuesta.error, operacion, registro, recuperacion: !!reenvio,
    };
    muestras.push(muestra);
    turno?.solicitudes.push(muestra);
    return muestra;
  };
  const fichaModelo = estado.contenido?.modelo;
  const origenModelo = values["ollama-url"] ? new URL(values["ollama-url"]) : null;
  const modelo = origenModelo && fichaModelo
    ? await inventarioModelo(origenModelo, fichaModelo.nombre) : null;
  let ramPico = totalmem() - freemem();
  let muestrasRam = 1;
  let vramPico: number | null = null;
  let muestrasVram = 0;
  let sondeoVram: Promise<void> | undefined;
  const medirRecursos = () => {
    ramPico = Math.max(ramPico, totalmem() - freemem());
    muestrasRam++;
    if (!sondeoVram && origenModelo && fichaModelo) {
      sondeoVram = modelosOllama(origenModelo, "/api/ps").then((modelos) => {
        if (!modelos) return;
        const uso = modelos.find((item) => item.name === fichaModelo.nombre);
        const vram = uso ? uso.size_vram : 0;
        if (typeof vram !== "number") return;
        vramPico = Math.max(vramPico ?? 0, vram);
        muestrasVram++;
      }).finally(() => { sondeoVram = undefined; });
    }
  };
  const inicio = Date.now();
  medirRecursos();
  const muestreo = setInterval(medirRecursos, 2000);
  let finCarga = inicio;
  let fin = inicio;
  try {
    if (perfil === "rr01") {
      const acciones: Accion[] = ["abrir", "guardar", "dialogo"];
      await Promise.all(clientes.map(async (cliente, i) => {
        for (let n = i; Date.now() < inicio + segundos * 1000; n++) {
          await usar(cliente, acciones[n % acciones.length], n);
        }
      }));
    } else {
      const mezcla: Accion[] = ["abrir", "guardar", "dialogo", "entregar"];
      await Promise.all(clientes.map((cliente, i) => usar(cliente, perfil === "rafaga" ? "dialogo" : mezcla[i % mezcla.length], i)));
    }
    finCarga = Date.now();
    if (perfil === "rafaga") {
      const sinAcuse = [...esperadas.values()].filter((turno) => recuperable(turno.solicitudes[0]));
      await Promise.all(sinAcuse.map(async (turno) => {
        const limite = performance.now() + esperaMs;
        let muestra = turno.solicitudes[0];
        do {
          const esperaServidor = muestra.reintentar_ms ?? (muestra.en_curso ? 1000 : 0);
          if (performance.now() + esperaServidor >= limite) return;
          if (esperaServidor > 0) await esperar(esperaServidor);
          const restante = Math.floor(limite - performance.now());
          if (restante <= 0) return;
          muestra = await usar(turno.cliente, "dialogo", 0, turno, restante);
        } while (muestra.en_curso);
      }));
    }
    fin = Date.now();
  } finally {
    clearInterval(muestreo);
    await sondeoVram;
  }
  const historiales = await Promise.all(clientes.map(async (cliente) => {
    const vista = await peticion("/intentos/" + cliente.intento, "GET", undefined, cliente.cookie);
    const { estudiante_id, intento_id, turnos = [], mediciones = [], entrega } = vista.contenido ?? {};
    const propia = vista.http === 200 && estudiante_id === cliente.identidad && intento_id === cliente.intento;
    const ajena = vista.http === 200 && ((estudiante_id !== undefined && estudiante_id !== cliente.identidad)
      || (intento_id !== undefined && intento_id !== cliente.intento));
    return { cliente, propia, ajena, http: vista.http, turnos, mediciones, entrega };
  }));
  const accesosAjenos = perfil === "sesiones" && clientes.length > 1
    ? await Promise.all(clientes.map(async (cliente, indice) => {
      const ajeno = clientes[(indice + 1) % clientes.length].intento;
      const vista = await peticion("/intentos/" + ajeno, "GET", undefined, cliente.cookie);
      return { identidad: cliente.identidad, intento: ajeno, http: vista.http };
    })) : [];
  let duplicados = 0;
  let cruces = muestras.filter((fila) => fila.cruce).length
    + accesosAjenos.filter(({ http }) => http >= 200 && http < 300).length;
  const historialesPropios = new Set(historiales.filter((historial) => historial.propia).map(({ cliente }) => cliente.intento));
  const operacionesVistas = new Set<string>();
  const vistas = new Map<string, string[]>();
  const registros = new Map<string, string>();
  for (const { cliente, propia, ajena, turnos, mediciones, entrega } of historiales) {
    if (ajena) cruces++;
    for (const turno of turnos) {
      const esperada = esperadas.get(turno.operacion_id);
      if (operacionesVistas.has(turno.operacion_id)) duplicados++;
      operacionesVistas.add(turno.operacion_id);
      if (!propia || turno.intento_id !== cliente.intento
        || esperada?.cliente.intento !== cliente.intento || turno.pregunta !== esperada.texto) {
        cruces++;
        continue;
      }
      const confirmados = vistas.get(turno.operacion_id) ?? [];
      confirmados.push(turno.turno_id);
      vistas.set(turno.operacion_id, confirmados);
    }
    if (!propia) continue;
    for (const fila of mediciones) {
      if (registros.has(fila.medicion_id)) duplicados++;
      if (fila.intento_id !== cliente.intento) cruces++;
      else registros.set(fila.medicion_id, cliente.intento);
    }
    if (entrega) {
      if (entrega.intento_id !== cliente.intento) cruces++;
      if (registros.has(entrega.entrega_id)) duplicados++;
      if (entrega.intento_id === cliente.intento) registros.set(entrega.entrega_id, cliente.intento);
    }
  }
  const escrituras = muestras.filter((fila) => fila.accion === "guardar" || fila.accion === "entregar");
  const acuses = new Set(escrituras.flatMap((fila) => fila.confirmada && fila.registro ? [fila.registro] : []));
  let escriturasAusentes = 0;
  let escriturasNoConciliadas = 0;
  for (const fila of escrituras.filter((fila) => fila.confirmada)) {
    if (!historialesPropios.has(fila.intento)) {
      escriturasNoConciliadas++;
      continue;
    }
    const titular = fila.registro && registros.get(fila.registro);
    if (!titular) escriturasAusentes++;
    else if (titular !== fila.intento) cruces++;
  }
  const log = values.log ? readFileSync(values.log) : null;
  const logTruncado = log !== null && log.length < logInicio;
  const marcas = log?.subarray(logTruncado ? 0 : logInicio).toString("utf8") ?? "";
  const entregasConfirmadas = new Set(escrituras.flatMap((muestra) =>
    muestra.accion === "entregar" && muestra.confirmada && muestra.registro ? [muestra.registro] : []));
  const { eventos, evaluaciones, tomas, invalidos } = leerEventos(marcas, esperadas, entregasConfirmadas);
  const estadoTurnos = [...esperadas.values()].map(({ operacion, cliente, solicitudes }) => {
    const ids = vistas.get(operacion) ?? [];
    const acuse = solicitudes.find((muestra) => muestra.confirmada);
    const ultima = solicitudes.at(-1);
    const estado = conciliarTurno(historialesPropios.has(cliente.intento), ids, acuse, ultima);
    const tiempos = eventos.get(operacion) ?? marcasVacias();
    const recibido = tiempos.recibidas[0] ?? null;
    const iniciado = tiempos.iniciadas[0] ?? null;
    const primera = solicitudes[0];
    return {
      operacion, intento: cliente.intento, estado, acuse: !!acuse, acuse_turno_id: acuse?.registro ?? null,
      turnos: ids.length, confirmados: ids,
      enviado_ms: primera?.enviado_ms ?? null, recibido_ms: recibido, inicio_ms: iniciado,
      espera_ms: recibido !== null && iniciado !== null && iniciado >= recibido ? iniciado - recibido : null,
      http_inicial: primera?.http ?? null, latencia_inicial_ms: primera?.duracion_ms ?? null,
      latencia_hasta_acuse_ms: acuse && primera ? acuse.enviado_ms - primera.enviado_ms + acuse.duracion_ms : null,
      en_curso_observado: solicitudes.some((muestra) => muestra.en_curso),
      confirmado_ms: tiempos.confirmadas[0] ?? null,
      inferencia_inicio_ms: tiempos.inferencias[0]?.inicio ?? null,
      inferencia_fin_ms: tiempos.inferencias[0]?.fin ?? null,
      reintento: solicitudes.length > 1, tiempos,
    };
  });
  const esperas = estadoTurnos.flatMap((turno) => turno.espera_ms === null ? [] : [turno.espera_ms]);
  const inferencias = estadoTurnos.flatMap((turno) => turno.tiempos.inferencias);
  const inferenciasEvaluacion = [...evaluaciones.values()].flat();
  const resultados = Object.fromEntries((["abrir", "guardar", "dialogo", "entregar"] as const).map((accion) => {
    const filas = muestras.filter((muestra) => muestra.accion === accion && !muestra.recuperacion);
    const exitos = filas.filter((muestra) => muestra.confirmada);
    return [accion, { total: filas.length, completadas: exitos.length, errores: filas.length - exitos.length,
      p50_ms: percentil(exitos.map((fila) => fila.duracion_ms), .50),
      p95_ms: percentil(exitos.map((fila) => fila.duracion_ms), .95) }];
  }));
  const operacionesRr01 = (["abrir", "guardar", "dialogo"] as const).map((accion, i) => {
    const { total, completadas, p95_ms } = resultados[accion];
    const p95_propuesto_ms = [2000, 3000, 15000][i];
    const muestra_suficiente = total >= 100;
    return { accion, total, completadas, muestra_suficiente, p95_ms, p95_propuesto_ms,
      cumple_p95_propuesto: muestra_suficiente && p95_ms !== null ? p95_ms <= p95_propuesto_ms : null };
  });
  const totalRr01 = operacionesRr01.reduce((total, fila) => total + fila.total, 0);
  const completadasRr01 = operacionesRr01.reduce((total, fila) => total + fila.completadas, 0);
  const indicadoresTurno = Object.fromEntries(
    (["confirmado", "recuperado", "persistido_sin_acuse", "en_curso", "fallido", "perdido",
      "duplicado", "acuse_incompatible", "no_conciliado"] as const)
      .map((estado) => [estado, estadoTurnos.filter((turno) => turno.estado === estado).length]),
  );
  const informe = {
    perfil, caracter: "exploratorio", inicio, fin, fin_carga_inicial: finCarga, usuarios,
    carga_completa: usuarios === previstos && (perfil !== "rr01" || segundos === 900),
    equipo: { plataforma: platform(), arquitectura: arch(), cpu: cpus()[0]?.model, cpus: cpus().length,
      ram_bytes: totalmem(), ram_pico_sistema_bytes: ramPico, muestras_ram: muestrasRam },
    modelo: { ...fichaModelo, inventario: modelo, vram_pico_bytes: vramPico, muestras_vram: muestrasVram },
    telemetria: {
      registro_axum: !!values.log, log_truncado: logTruncado, eventos_invalidos: invalidos,
      recepciones_observadas: estadoTurnos.filter((turno) => turno.recibido_ms !== null).length,
      esperas_observadas: esperas.length, intervalos_entrevista: inferencias.length,
      intervalos_evaluacion: inferenciasEvaluacion.length, entregas_con_toma_observada: tomas.size,
      entregas_sin_intervalo_evaluacion: entregasConfirmadas.size - evaluaciones.size,
    },
    sesiones_abiertas: clientes.length, resultados,
    liberacion_rafaga_ms: perfil === "rafaga" ? inicio : null,
    referencia_rr01: perfil === "rr01" ? {
      muestra_suficiente: operacionesRr01.every((fila) => fila.muestra_suficiente), segundos,
      operaciones: operacionesRr01, completadas_propuesto_pct: 99,
      completadas_pct: totalRr01 ? completadasRr01 / totalRr01 * 100 : null,
    } : null,
    conciliacion: { historiales: historiales.length, historiales_propios: historialesPropios.size,
      historiales_inaccesibles: historiales.filter(({ http }) => http !== 200).length, cruces, duplicados,
      historiales_con_acuse_incompleto: historiales.filter(({ http, propia, ajena }) =>
        http === 200 && !propia && !ajena).length,
      accesos_ajenos_denegados: accesosAjenos.filter(({ http }) => http === 403 || http === 404).length,
      perdidos: indicadoresTurno.perdido, recuperados: indicadoresTurno.recuperado,
      persistidos_sin_acuse: indicadoresTurno.persistido_sin_acuse, estados_turno: indicadoresTurno,
      turnos_con_acuse_en_curso: estadoTurnos.filter((turno) => turno.en_curso_observado).length,
      escrituras_ausentes: escriturasAusentes, escrituras_no_conciliadas: escriturasNoConciliadas,
      escrituras_persistidas_sin_acuse: [...registros.keys()].filter((id) => !acuses.has(id)).length,
      pico_inferencias_observadas: pico([...inferencias, ...inferenciasEvaluacion], inicio, fin),
      pico_inferencias_entrevista_observadas: pico(inferencias, inicio, fin),
      esperas_p50_ms: percentil(esperas, .50), esperas_p95_ms: percentil(esperas, .95),
      eventos_sqlite_ocupada: values.log ? (marcas.match(/database is locked|database is busy/gi) ?? []).length : null,
    },
    historiales: historiales.map(({ cliente, propia, http, turnos, mediciones, entrega }) => ({
      identidad: cliente.identidad, intento: cliente.intento, http, propia,
      turnos: turnos.length, mediciones: mediciones.length, entrega: entrega?.entrega_id ?? null,
    })),
    accesos_ajenos: accesosAjenos, muestras, turnos: estadoTurnos,
    evaluaciones_observadas: [...entregasConfirmadas].map((entrega_id) => ({
      entrega_id, tomas_ms: tomas.get(entrega_id) ?? [], inferencias: evaluaciones.get(entrega_id) ?? [],
    })),
  };
  writeFileSync(values.salida, JSON.stringify(informe, null, 2) + "\n", { flag: "wx" });
  console.log(JSON.stringify({ perfil, usuarios, resultados, conciliacion: informe.conciliacion }));
}

function asignar(ruta: string, cantidad: number): string[] {
  const db = new DatabaseSync(ruta);
  const prefijo = "carga-" + randomUUID().slice(0, 12);
  const identidades = Array.from({ length: cantidad }, (_, i) => prefijo + "-" + (i + 1));
  try {
    assert.ok(db.prepare("SELECT 1 FROM rubricas_activas_ensayo WHERE actividad_id='lr-ensayo'").get(),
      "Prepara la base de prueba con --rubrica-ensayo");
    db.exec("PRAGMA busy_timeout=5000; PRAGMA foreign_keys=ON; BEGIN IMMEDIATE");
    const identidad = db.prepare("INSERT INTO identidades VALUES (?1,'estudiante')");
    const asignacion = db.prepare("INSERT INTO asignaciones VALUES ('lr-ensayo',?1,'docente-demo')");
    for (const id of identidades) {
      identidad.run(id);
      asignacion.run(id);
    }
    db.exec("COMMIT");
    return identidades;
  } finally {
    db.close();
  }
}

async function consultar(
  origen: URL, ruta: string, milisegundos: number, metodo: string, contenido?: object, cookie?: string, clave?: string,
): Promise<Respuesta> {
  const headers = new Headers();
  if (cookie) headers.set("Cookie", cookie);
  if (clave) headers.set("Idempotency-Key", clave);
  if (contenido) headers.set("Content-Type", "application/json");
  try {
    const respuesta = await fetch(new URL("/api" + ruta, origen), {
      method: metodo, headers, body: contenido && JSON.stringify(contenido),
      redirect: "error", signal: AbortSignal.timeout(milisegundos),
    });
    const retryAfter = respuesta.headers.get("retry-after");
    const reintentarMs = retryAfter && /^\d+$/.test(retryAfter) ? Number(retryAfter) * 1000 : undefined;
    return { http: respuesta.status, contenido: await respuesta.json().catch(() => null),
      cookie: respuesta.headers.get("set-cookie")?.split(";")[0] ?? null,
      reintentar_ms: Number.isSafeInteger(reintentarMs) ? reintentarMs : undefined };
  } catch (error) {
    return { http: 0, contenido: null, cookie: null,
      error: error instanceof DOMException && error.name === "TimeoutError" ? "tiempo_agotado" : "transporte" };
  }
}

function recuperable(muestra: Muestra | undefined): boolean {
  return !!muestra && !muestra.cruce
    && (muestra.en_curso || [0, 429, 500, 502, 503, 504].includes(muestra.http));
}

function conciliarTurno(historialPropio: boolean, ids: string[], acuse?: Muestra, ultima?: Muestra) {
  if (!historialPropio) return "no_conciliado";
  if (ids.length > 1) return "duplicado";
  if (ids.length === 1) {
    if (!acuse) return "persistido_sin_acuse";
    if (acuse.registro !== ids[0]) return "acuse_incompatible";
    return acuse.recuperacion ? "recuperado" : "confirmado";
  }
  if (acuse) return "perdido";
  return ultima?.en_curso ? "en_curso" : "fallido";
}

function idTurno(cliente: Cliente, clave: string, texto: string): string {
  const huella = createHash("sha256").update("POST turnos\n").update(JSON.stringify(texto)).digest("hex");
  const hash = createHash("sha256");
  for (const parte of [cliente.identidad, cliente.intento, clave, huella]) {
    const bytes = Buffer.from(parte);
    const longitud = Buffer.alloc(8);
    longitud.writeBigUInt64BE(BigInt(bytes.length));
    hash.update(longitud).update(bytes);
  }
  return hash.digest("hex").slice(0, 32);
}

type Intervalo = { inicio: number; fin: number };
type Marcas = { recibidas: number[]; iniciadas: number[]; confirmadas: number[]; inferencias: Intervalo[] };

function marcasVacias(): Marcas {
  return { recibidas: [], iniciadas: [], confirmadas: [], inferencias: [] };
}

function leerEventos(log: string, esperadas: Map<string, TurnoEsperado>, entregas: Set<string>) {
  const eventos = new Map<string, Marcas>();
  const evaluaciones = new Map<string, Intervalo[]>();
  const tomas = new Map<string, number[]>();
  let invalidos = 0;
  for (const linea of log.split(/\r?\n/)) {
    const marca = /^(?:[\w.-]+\s*\|\s*)?(?:\d{4}-\d\d-\d\dT\S+\s+)?(SIRCLE_[A-Z_]+)\s+(.*)$/.exec(linea);
    if (!marca) continue;
    const [, evento, datos] = marca;
    if (!["SIRCLE_TURNO_RECIBIDO", "SIRCLE_TRABAJADOR_OPERACION", "SIRCLE_TURNO_CONFIRMADO", "SIRCLE_INFERENCIA",
      "SIRCLE_EVALUACION_TOMA", "SIRCLE_INFERENCIA_EVALUACION"].includes(evento)) continue;
    try {
      const fila = JSON.parse(datos);
      const { inicio_unix_ms: inicio, fin_unix_ms: fin, recibido_unix_ms: recibido, confirmado_unix_ms: confirmado } = fila;
      if (evento === "SIRCLE_EVALUACION_TOMA" || evento === "SIRCLE_INFERENCIA_EVALUACION") {
        const id = fila.entrega_id;
        if (!entregas.has(id)) continue;
        if (evento === "SIRCLE_EVALUACION_TOMA" && Number.isSafeInteger(inicio)) {
          tomas.set(id, [...(tomas.get(id) ?? []), inicio]);
        } else if (evento === "SIRCLE_INFERENCIA_EVALUACION" && Number.isSafeInteger(inicio) && Number.isSafeInteger(fin) && fin >= inicio) {
          evaluaciones.set(id, [...(evaluaciones.get(id) ?? []), { inicio, fin }]);
        } else {
          invalidos++;
        }
        continue;
      }
      if (!esperadas.has(fila.operacion_id)) continue;
      const tiempos = eventos.get(fila.operacion_id) ?? marcasVacias();
      if (evento === "SIRCLE_TURNO_RECIBIDO" && Number.isSafeInteger(recibido)) tiempos.recibidas.push(recibido);
      else if (evento === "SIRCLE_TRABAJADOR_OPERACION" && Number.isSafeInteger(inicio)) tiempos.iniciadas.push(inicio);
      else if (evento === "SIRCLE_TURNO_CONFIRMADO" && Number.isSafeInteger(confirmado)) tiempos.confirmadas.push(confirmado);
      else if (evento === "SIRCLE_INFERENCIA" && Number.isSafeInteger(inicio) && Number.isSafeInteger(fin) && fin >= inicio) {
        tiempos.inferencias.push({ inicio, fin });
      } else {
        invalidos++;
      }
      eventos.set(fila.operacion_id, tiempos);
    } catch {
      invalidos++;
    }
  }
  return { eventos, evaluaciones, tomas, invalidos };
}

function percentil(valores: number[], p: number): number | null {
  if (!valores.length) return null;
  const ordenados = valores.toSorted((a, b) => a - b);
  return ordenados[Math.ceil(ordenados.length * p) - 1];
}

function pico(inferencias: Intervalo[], desde: number, hasta: number): number | null {
  const cambios: [number, number][] = [];
  for (const intervalo of inferencias) {
    const inicio = Math.max(intervalo.inicio, desde);
    const fin = Math.min(intervalo.fin, hasta);
    if (fin > inicio) cambios.push([inicio, 1], [fin, -1]);
  }
  if (!cambios.length) return null;
  cambios.sort(([a, deltaA], [b, deltaB]) => a - b || deltaA - deltaB);
  let activas = 0;
  let maxima = 0;
  for (const [, delta] of cambios) {
    activas += delta;
    maxima = Math.max(maxima, activas);
  }
  return maxima;
}

type ModeloOllama = { name: string; digest: string; size?: number; size_vram?: number;
  details?: { quantization_level?: string } };

async function modelosOllama(origen: URL, ruta: "/api/tags" | "/api/ps"): Promise<ModeloOllama[] | null> {
  try {
    const respuesta = await fetch(new URL(ruta, origen), { redirect: "error", signal: AbortSignal.timeout(3000) });
    if (!respuesta.ok) return null;
    const { models } = await respuesta.json();
    return Array.isArray(models) ? models.filter((modelo) => typeof modelo?.name === "string") : null;
  } catch {
    return null;
  }
}

async function inventarioModelo(origen: URL, nombre: string) {
  const [tags, ps] = await Promise.all([modelosOllama(origen, "/api/tags"), modelosOllama(origen, "/api/ps")]);
  const ficha = tags?.find((modelo) => modelo.name === nombre);
  const uso = ps?.find((modelo) => modelo.name === nombre);
  return ficha ? { nombre, digest: ficha.digest, cuantizacion: ficha.details?.quantization_level,
    memoria_modelo_bytes: uso?.size, vram_bytes: uso?.size_vram } : null;
}
