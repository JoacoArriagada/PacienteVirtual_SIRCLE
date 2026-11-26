"""Ejecuta E22 con el contenedor de Ollama de T013 y conserva la base."""

import argparse
from contextlib import contextmanager
from pathlib import Path
import json
import os
import signal
import sqlite3
import subprocess
import time
from urllib.error import HTTPError
from urllib.request import ProxyHandler, Request, build_opener


RAIZ = Path(__file__).resolve().parents[3]
SIRCLE = RAIZ / "backend/target/debug/sircle"
MODELO = "qwen3.5:4b"
DIGEST = "d8b0f5e9760cd1682034f292d7ef72ec46f432149be0df7574bf2d6e92e38c04"
CONTENEDOR = "sircle-ollama-h1"
OLLAMA = "http://127.0.0.1:11434"
AXUM = "http://127.0.0.1:18081"
CLIENTE = build_opener(ProxyHandler({}))


def comando(*argumentos):
    return subprocess.check_output(argumentos, cwd=RAIZ, text=True).strip()


def http(ruta, *, datos=None, cookie=None, clave=None, metodo=None, servidor=AXUM, limite=10):
    cabeceras = {"Content-Type": "application/json"}
    if cookie:
        cabeceras["Cookie"] = cookie
    if clave:
        cabeceras["Idempotency-Key"] = clave
    peticion = Request(servidor + ruta, None if datos is None else json.dumps(datos).encode(), cabeceras, method=metodo)
    try:
        respuesta = CLIENTE.open(peticion, timeout=limite)
    except HTTPError as error:
        respuesta = error
    with respuesta:
        return respuesta.status, json.load(respuesta), respuesta.headers


def entrar(identidad):
    codigo, datos, headers = http("/api/sesiones", datos={"identidad": identidad})
    assert codigo == 201, datos
    return headers["Set-Cookie"].split(";", 1)[0]


def esperar(comprobar, segundos, intervalo=0.25):
    limite = time.monotonic() + segundos
    while time.monotonic() < limite:
        valor = comprobar()
        if valor:
            return valor
        time.sleep(intervalo)
    raise RuntimeError("Se agotó el plazo de espera de E22")


@contextmanager
def servicio(db, log):
    with log.open("w") as salida:
        proceso = subprocess.Popen([SIRCLE, "servir", "--db", db, "--direccion", "127.0.0.1:18081"],
                                   cwd=RAIZ, stdout=salida, stderr=subprocess.STDOUT, start_new_session=True,
                                   env={**os.environ, "OLLAMA_URL": OLLAMA})
        try:
            esperar(lambda: proceso.poll() is not None or "SIRCLE API en" in log.read_text(), 30)
            assert proceso.poll() is None, log.read_text()
            yield
        finally:
            if proceso.poll() is None:
                os.killpg(proceso.pid, signal.SIGTERM)
                try:
                    proceso.wait(timeout=20)
                except subprocess.TimeoutExpired:
                    os.killpg(proceso.pid, signal.SIGKILL)
                    proceso.wait(timeout=5)


def escribir(ruta, datos):
    ruta.write_text(json.dumps(datos, ensure_ascii=False, indent=2) + "\n")


def filas(db, intento):
    with sqlite3.connect(db) as con:
        con.row_factory = sqlite3.Row
        toma = con.execute("SELECT id FROM mediciones WHERE intento_id=?1 ORDER BY id", (intento,)).fetchall()
        entrega = con.execute("SELECT id,revision FROM entregas WHERE intento_id=?1", (intento,)).fetchone()
        trabajo = con.execute("""SELECT w.* FROM trabajos_evaluacion w JOIN entregas e ON e.id=w.entrega_id
                                 WHERE e.intento_id=?1""", (intento,)).fetchone()
        post = con.execute("SELECT estado FROM post_ensayo WHERE intento_id=?1", (intento,)).fetchone()
        resultado = con.execute("""SELECT v.id,v.operacion_id FROM evaluaciones v JOIN entregas e ON e.id=v.entrega_id
                                   WHERE e.intento_id=?1""", (intento,)).fetchall()
        acuses = con.execute("SELECT count(*) FROM operaciones WHERE ambito=?1 AND clave='e22-entrega'", (intento,)).fetchone()[0]
    return {
        "tomas": [r["id"] for r in toma], "entrega": dict(entrega) if entrega else None,
        "trabajo": dict(trabajo) if trabajo else None, "post": post["estado"] if post else None,
        "evaluaciones": [dict(r) for r in resultado], "acuses_entrega": acuses,
    }


def preparar(db):
    comando(SIRCLE, "preparar", "--db", db, "--fuentes", "datos")
    fuente = RAIZ / "backend/tests/fixtures/rubricas/lr-v1.json"
    borrador = json.loads(comando(SIRCLE, "cargar-rubrica-ensayo", "--db", db, "--archivo", fuente))
    rubrica = json.loads(fuente.read_text())
    rubrica["aprobacion"] = {"tipo": "ensayo", "docente_id": "docente-demo", "fecha": "2026-11-26T00:00:00Z",
                            "motivo": "Constancia ficticia del ensayo E22", "codigo": borrador["codigo"],
                            "version": borrador["version"], "sha256": borrador["sha256"]}
    aprobacion = db.parent / "rubrica-ensayo.json"
    escribir(aprobacion, rubrica)
    aprobada = json.loads(comando(SIRCLE, "cargar-rubrica-ensayo", "--db", db, "--archivo", aprobacion))
    comando(SIRCLE, "habilitar-rubrica-ensayo", "--db", db, "--rubrica-id", str(aprobada["id"]))


def en_caida(db, destino):
    with servicio(db, db.parent / "caida.log"):
        cookie = entrar("estudiante-lr")
        codigo, alta, _ = http("/api/intentos", datos={"actividad_id": "lr-ensayo"}, cookie=cookie, clave="e22-alta")
        assert codigo == 201, alta
        intento = alta["intento_id"]
        ruta = "/api/intentos/" + intento
        comando(SIRCLE, "resolver-pre-ensayo", "--db", db, "--intento", intento)
        assert filas(db, intento)["tomas"] == []
        codigo, toma, _ = http(ruta + "/mediciones", datos={"instrumento": "presion", "accion": "toma_completa"},
                              cookie=cookie, clave="e22-presion")
        assert codigo == 201, toma
        codigo, borrador, _ = http(ruta + "/razonamiento", metodo="PUT", cookie=cookie, clave="e22-hipotesis", datos={
            "revision_esperada": toma["revision"], "hipotesis": [{
                "rol": "principal", "titulo": "Presión sistólica elevada en esta toma",
                "explicacion": "La lectura instrumental es 148/85 mmHg y permite describir la toma actual.",
                "evidencias": [{"tipo": "medicion", "origen_id": toma["medicion_id"]}],
            }],
        })
        assert codigo == 200, borrador
        peticion = {"revision_esperada": borrador["revision"]}
        codigo, entrega, _ = http(ruta + "/entrega", datos=peticion, cookie=cookie, clave="e22-entrega")
        repetido_http, repetido, _ = http(ruta + "/entrega", datos=peticion, cookie=cookie, clave="e22-entrega")
        assert codigo == 201 and repetido_http == 200 and entrega == repetido
        comando(SIRCLE, "resolver-post-ensayo", "--db", db, "--intento", intento, "--estado", "omitido")

        def hubo_fallo():
            estado = filas(db, intento)
            return estado if estado["trabajo"]["estado"] == "pendiente" and estado["trabajo"]["ultimo_error"] else None

        antes = esperar(hubo_fallo, 12)
        assert antes["tomas"] == [toma["medicion_id"]] and antes["evaluaciones"] == []
        assert antes["entrega"]["id"] == entrega["entrega_id"] == antes["trabajo"]["entrega_id"]
        assert antes["post"] == "omitido" and antes["acuses_entrega"] == 1
        datos = {"antes": antes, "toma": toma, "borrador": borrador, "entrega": entrega, "reenvio_http": repetido_http}
        escribir(destino / "en-caida.json", datos)
        return intento, datos


def generar():
    def listo():
        try:
            codigo, inventario, _ = http("/api/tags", servidor=OLLAMA, limite=3)
            return codigo == 200 and any(m["name"] == MODELO and m["digest"] == DIGEST for m in inventario["models"])
        except OSError:
            return False
    esperar(listo, 45)
    codigo, generada, _ = http("/api/chat", servidor=OLLAMA, limite=170, datos={
        "model": MODELO, "stream": False, "think": False, "keep_alive": "10m",
        "options": {"num_ctx": 4096, "num_predict": 12, "temperature": 0},
        "messages": [{"role": "user", "content": "Responde solo con la palabra Listo."}],
    })
    assert codigo == 200 and generada["done"] and generada["message"]["content"].strip(), generada
    return generada


def principal(db, destino, reanudar):
    destino.mkdir(parents=True, exist_ok=True)
    if reanudar:
        caida = json.loads((destino / "en-caida.json").read_text())
        intento = caida["entrega"]["intento_id"]
        actual = filas(db, intento)
        for campo in ("tomas", "entrega", "post", "evaluaciones", "acuses_entrega"):
            assert actual[campo] == caida["antes"][campo], campo
        assert actual["trabajo"]["entrega_id"] == caida["antes"]["trabajo"]["entrega_id"]
        assert actual["trabajo"]["estado"] == "pendiente"
        recuperar(db, destino, intento, caida)
        return
    assert not db.exists(), db
    db.parent.mkdir(parents=True, exist_ok=True)
    preparar(db)
    assert comando("docker", "inspect", CONTENEDOR, "--format", "{{.State.Running}}") == "true"
    comando("docker", "stop", CONTENEDOR)
    try:
        assert comando("docker", "inspect", CONTENEDOR, "--format", "{{.State.Running}}") == "false"
        try:
            http("/api/tags", servidor=OLLAMA, limite=3)
        except OSError:
            print("Ollama detenido y puerto local indisponible.", flush=True)
        else:
            raise RuntimeError("El puerto de prueba sigue atendiendo tras detener Ollama")
        intento, caida = en_caida(db, destino)
        print("Una toma, un acuse, post de ensayo omitido y un trabajo pendiente.", flush=True)
    finally:
        comando("docker", "start", CONTENEDOR)
    recuperar(db, destino, intento, caida)


def recuperar(db, destino, intento, caida):
    generada = generar()
    print("Ollama reinició y generó la respuesta de prueba. Se reanuda la cola.", flush=True)
    log = db.parent / "recuperacion.log"
    with servicio(db, log):
        def termino():
            estado = filas(db, intento)
            return estado if estado["trabajo"]["estado"] == "terminado" else None
        try:
            despues = esperar(termino, 620, 0.5)
        except RuntimeError:
            print(log.read_text()[-4000:], flush=True)
            raise
        assert len(despues["evaluaciones"]) == 1
        assert despues["tomas"] == caida["antes"]["tomas"] and despues["entrega"] == caida["antes"]["entrega"]
        assert despues["trabajo"]["entrega_id"] == caida["antes"]["trabajo"]["entrega_id"]
        assert despues["post"] == "omitido" and despues["acuses_entrega"] == 1
        assert despues["trabajo"]["toma_id"] is None and despues["trabajo"]["ultimo_error"] is None
        evaluacion = despues["evaluaciones"][0]
        docente = entrar("docente-demo")
        codigo, traza, _ = http(f"/api/evaluaciones/{evaluacion['id']}/traza", cookie=docente)
        assert codigo == 200 and traza["invocacion"]["digest"] == DIGEST
        assert traza["mensaje"]["operacion_id"] == traza["invocacion"]["operacion_id"] == evaluacion["operacion_id"]
        assert traza["entrada"]["hipotesis"][0]["evidencias"] == despues["tomas"]
        _, modelos, _ = http("/api/ps", servidor=OLLAMA)
        modelo = next(m for m in modelos["models"] if m["name"] == MODELO)
        assert modelo["size_vram"] == 0
    marcas = [line for p in (db.parent / "caida.log", log) for line in p.read_text().splitlines()
              if line.startswith("SIRCLE_EVALUACION_")]
    escribir(destino / "recuperacion-real.json", {
        **caida, "despues": despues, "generacion_recuperada": generada, "traza_docente": traza,
        "modelo_cpu": modelo, "marcas_axum": marcas, "base_retenida": str(db),
    })
    print("Se confirmó una evaluación:", evaluacion["id"], flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--db", required=True, type=Path)
    parser.add_argument("--destino", required=True, type=Path)
    parser.add_argument("--reanudar", action="store_true")
    argumentos = parser.parse_args()
    principal(argumentos.db.resolve(), argumentos.destino.resolve(), argumentos.reanudar)
