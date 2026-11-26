"""Repite E17 y las entrevistas cruzadas con los trabajadores y Ollama locales."""

from concurrent.futures import ThreadPoolExecutor
import importlib.util
import json
import os
from pathlib import Path
import sqlite3
import subprocess
import sys
from tempfile import TemporaryDirectory
from threading import Barrier


sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location("e22", Path(__file__).with_name("ejecutar-e22.py"))
e22 = importlib.util.module_from_spec(spec)
spec.loader.exec_module(e22)


def preparar(db):
    e22.comando(e22.SIRCLE, "preparar", "--db", db, "--fuentes", "datos")
    with sqlite3.connect(db) as con:
        con.execute("PRAGMA foreign_keys=ON")
        con.execute("UPDATE actividades SET habilitada=1 WHERE id IN ('nannu-ensayo','dan-ensayo')")
        con.executemany("INSERT INTO asignaciones VALUES (?,?,'docente-demo')", [
            ("nannu-ensayo", "estudiante-lr"),
            ("dan-ensayo", "estudiante-lr"),
            ("lr-ensayo", "estudiante-sin-asignacion"),
        ])


def rust(db, log, archivo, prueba, prefijo):
    proceso = subprocess.run(["cargo", "test", "--manifest-path", "backend/Cargo.toml", "--locked", "--offline",
                              "--test", archivo, prueba, "--", "--ignored", "--nocapture"], cwd=e22.RAIZ,
                             env={**os.environ, "SIRCLE_URL": e22.AXUM, f"{prefijo}_DB": str(db), f"{prefijo}_LOG": str(log)},
                             stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    if proceso.returncode:
        print(proceso.stdout, log.read_text()[-5000:], flush=True)
        raise SystemExit(proceso.returncode)
    salida = next(json.loads(line.removeprefix("EVIDENCIA ")) for line in proceso.stdout.splitlines() if line.startswith("EVIDENCIA "))
    salida["regresion_rust"] = next(line for line in proceso.stdout.splitlines() if line.startswith("test result:"))
    return salida


def cruzar(db, log):
    titular = e22.entrar("estudiante-lr")
    docente = e22.entrar("docente-demo")
    casos = [
        ("P02", "nannu-ensayo", "Acompañante", "¿La ha visto toser y expulsar flema?", "p02-tos-productiva"),
        ("P03", "dan-ensayo", "Dan Sparks", "¿Hace cuánto comenzó la dificultad para respirar?", "p03-inicio"),
    ]
    altas = []
    for caso, actividad, *_ in casos:
        codigo, intento, _ = e22.http("/api/intentos", datos={"actividad_id": actividad}, cookie=titular, clave=f"t065-{caso}")
        assert codigo == 201 and intento["caso"] == caso
        e22.comando(e22.SIRCLE, "resolver-pre-ensayo", "--db", db, "--intento", intento["intento_id"])
        altas.append(intento)
    barrera = Barrier(2)

    def enviar(pareja):
        caso, intento = pareja
        codigo, _, voz, texto, hecho = caso
        ruta = "/api/intentos/" + intento["intento_id"]
        barrera.wait(timeout=5)
        estado, turno, _ = e22.http(ruta + "/turnos", datos={"texto": texto, "destinatario": voz},
                                    cookie=titular, clave="t065-pregunta", limite=205)
        assert estado == 201 and turno["hablante"] == voz, turno
        estado, traza, _ = e22.http(f"{ruta}/turnos/{turno['turno_id']}/traza", cookie=docente)
        assert estado == 200 and traza["hecho"]["id"] == hecho and traza["mensaje"]["caso"] == codigo
        assert traza["mensaje"]["intento_id"] == turno["intento_id"] == intento["intento_id"]
        assert traza["invocacion"]["digest"] == e22.DIGEST
        return {"ruta": ruta, "alta": intento, "turno": turno, "traza": traza}

    with ThreadPoolExecutor(max_workers=2) as pool:
        registros = list(pool.map(enviar, zip(casos, altas)))
    for registro in registros:
        _, vista, _ = e22.http(registro["ruta"], cookie=e22.entrar("estudiante-lr"))
        assert vista["turnos"] == [registro["turno"]]
    ids = {r["turno"]["operacion_id"] for r in registros}
    assert len(ids) == 2
    marcas = {}
    for etiqueta in ("SIRCLE_TURNO_RECIBIDO", "SIRCLE_TURNO_CONFIRMADO"):
        marcas[etiqueta] = []
        for linea in log.read_text().splitlines():
            if linea.startswith(etiqueta + " "):
                marca = json.loads(linea.partition(" ")[2])
                if marca["operacion_id"] in ids:
                    marcas[etiqueta].append(marca)
        assert len(marcas[etiqueta]) == 2
    solapamiento = min(m["confirmado_unix_ms"] for m in marcas["SIRCLE_TURNO_CONFIRMADO"]) - max(
        m["recibido_unix_ms"] for m in marcas["SIRCLE_TURNO_RECIBIDO"])
    assert solapamiento > 0, marcas
    return {"operaciones": registros, "marcas": marcas, "solapamiento_ms": solapamiento}


def ejecutar(destino):
    destino.mkdir(parents=True, exist_ok=True)
    bloques = [
        ("lr-e17", lambda db, log: rust(db, log, "agente_lr", "aisla_dos_preguntas_simultaneas", "SIRCLE_T052")),
        ("nannu-reinicio", lambda db, log: rust(db, log, "agente_nannu", "retoma_turno_del_familiar", "SIRCLE_T053")),
        ("p02-p03", cruzar),
    ]
    for nombre, prueba in bloques:
        with TemporaryDirectory(prefix=f"sircle-t065-{nombre}-") as tmp:
            db, log = Path(tmp) / "intentos.sqlite", Path(tmp) / "axum.log"
            preparar(db)
            with e22.servicio(db, log):
                datos = prueba(db, log)
                codigo, modelos, _ = e22.http("/api/ps", servidor=e22.OLLAMA)
                cpu = next(m for m in modelos["models"] if m["name"] == e22.MODELO)
                assert codigo == 200 and cpu["size_vram"] == 0
                datos["modelo_cpu"] = cpu
                e22.escribir(destino / f"{nombre}.json", datos)
        print(f"{nombre} verificado", flush=True)


if __name__ == "__main__":
    ejecutar(Path(sys.argv[1]).resolve())
