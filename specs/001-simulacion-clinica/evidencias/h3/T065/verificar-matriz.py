"""Coteja los README, archivos de implementación y resultados de Cargo del hito 3."""

import csv
import hashlib
import json
from pathlib import Path
import re
import sys


DESTINO = Path(__file__).resolve().parent
RAIZ = DESTINO.parents[4]
CONTRATO = RAIZ / "specs/001-simulacion-clinica/contracts/agentes.md"
CAMPOS = ("Propósito", "Justificación", "Fuentes autorizadas",
          "Comportamiento esperado", "Implementación", "Prueba")
EXTERNAS = {
    "aisla_dos_preguntas_simultaneas": "lr-e17.json",
    "retoma_turno_del_familiar": "nannu-reinicio.json",
}


def enlaces(texto, directorio):
    return [(directorio / ruta.split("#", 1)[0]).resolve()
            for ruta in re.findall(r"\]\(<?([^)>]+)>?\)", texto)]


def matriz(log):
    locales = set(re.findall(r"(?m)^test (\w+) \.\.\. ok$", log))
    filas = []
    agente = None
    for linea in CONTRATO.read_text().splitlines():
        cabecera = re.match(r"Directorio \x60backend/src/agentes/(\w+)/\x60", linea)
        if cabecera:
            agente = cabecera[1]
        if not re.match(r"^\| \x60\d\d_", linea):
            continue
        modulo, _, implementacion, prueba = (celda.strip() for celda in linea.split("|")[1:5])
        modulo = modulo.strip("\x60").rstrip("/")
        archivo = (RAIZ / "backend/src" / implementacion.strip("\x60")).resolve()
        caso = prueba.split("\x60")[1]
        readme = RAIZ / "backend/src/agentes" / agente / modulo / "README.md"
        contenido = readme.read_text()
        notas = {campo: next(l for l in contenido.splitlines() if l.startswith(f"**{campo}.**")) for campo in CAMPOS}
        assert all(p.is_file() for p in enlaces(contenido, readme.parent)), readme
        assert archivo in enlaces(notas["Implementación"], readme.parent) and archivo.is_file(), readme
        assert f"\x60{caso}\x60" in notas["Prueba"], readme
        candidatos = enlaces(notas["Prueba"], readme.parent)
        archivo_prueba = next(p for p in candidatos if p.suffix == ".rs" and re.search(rf"\bfn {caso}\(", p.read_text()))
        if caso in EXTERNAS:
            detalle = json.loads((DESTINO / EXTERNAS[caso]).read_text())
            assert detalle["regresion_rust"].startswith("test result: ok. 1 passed"), caso
            evidencia = EXTERNAS[caso]
        else:
            assert caso in locales, caso
            evidencia = "rust.txt"
        filas.append((agente, modulo, readme.relative_to(RAIZ), archivo.relative_to(RAIZ),
                      archivo_prueba.relative_to(RAIZ), caso, prueba.split(" · ", 1)[1], evidencia))
    assert len(filas) == 32 and len({(fila[0], fila[1]) for fila in filas}) == 32
    assert {agente: sum(f[0] == agente for f in filas) for agente in {f[0] for f in filas}} == {
        "lr": 8, "nannu": 8, "dan_sparks": 8, "evaluador": 8,
    }
    with (DESTINO / "matriz.tsv").open("w", newline="") as salida:
        escritor = csv.writer(salida, delimiter="\t", lineterminator="\n")
        escritor.writerow(("agente", "nucleo", "readme", "implementacion", "archivo_de_prueba",
                            "caso_rust", "escenario", "resultado_verificado"))
        escritor.writerows(filas)
    print(f"{len(filas)} correspondencias verificadas")


def evidencias(log):
    return [json.loads(linea.removeprefix("EVIDENCIA "))
            for linea in log.splitlines() if linea.startswith("EVIDENCIA ")]


def instrucciones(invocacion):
    sistema = next(m["content"] for m in invocacion["solicitud"]["messages"] if m["role"] == "system")
    return hashlib.sha256(sistema.encode()).hexdigest()


def guardar(nombre, datos):
    (DESTINO / nombre).write_text(json.dumps(datos, ensure_ascii=False, indent=2) + "\n")


def recorridos(log_rust, log_e12):
    rechazo = next(e for e in evidencias(log_rust) if "origen_candidato" in e)
    cruces = json.loads((DESTINO / "p02-p03.json").read_text())
    assert cruces["solapamiento_ms"] > 0
    entrevista = next(e["traza"] for e in cruces["operaciones"] if e["alta"]["caso"] == "P02")
    fuente = rechazo["origen_candidato"]
    ruta = Path("datos/medsp1000") / fuente["fuente"]
    texto = (RAIZ / ruta).read_text().splitlines()[fuente["linea"] - 1]
    assert fuente["pasaje"] in texto
    assert rechazo["motivo"] == "seleccion_fuera_de_hablante" and rechazo["turnos"] == 0
    assert fuente["id"] == rechazo["candidato_cruzado"] == entrevista["hecho"]["id"]
    assert fuente["version"] == rechazo["intentado"]["version"] == entrevista["mensaje"]["version"]
    assert fuente["hablante"] == "Acompañante" and rechazo["intentado"]["hablante"] == "Nannu"

    calibraciones = evidencias(log_e12)
    assert len(calibraciones) == 2
    assert all(e["ollama_ps"]["size_vram"] == 0 and
               all(c["juicio_coincide"] and c["cita_coincide"] for c in e["cotejo"])
               for e in calibraciones)
    tres = next(e for e in calibraciones if sum(c["observacion"]["juicio"] == "acierto" for c in e["cotejo"]) == 3)
    uno = next(e for e in calibraciones if sum(c["observacion"]["juicio"] == "acierto" for c in e["cotejo"]) == 1)
    assert tres["acuse"]["rubrica_sha256"] == uno["acuse"]["rubrica_sha256"]
    guardar("e12-repetido.json", {"tres": tres, "uno": uno})
    criterio = uno["cotejo"][1]["observacion"]
    informe = uno["entradas_clinicas"]["informe"]
    ruta_ecg = Path("datos/medsp1000/mededportal_10181/scenario1/environment_controller/Case A-Asthma.md")
    pasaje = (RAIZ / ruta_ecg).read_text().splitlines()[3]
    assert "initial EKG (sinus tachycardia)" in pasaje
    assert criterio["criterio_id"] == "hipotesis-2" and criterio["juicio"] == "error"
    assert informe["informe_id"] in criterio["evidencias"]
    assert informe["resultado"] == "taquicardia sinusal"
    invocacion = uno["traza"]["invocacion"]
    cruda = json.loads(invocacion["respuesta_cruda"])["resultados"][criterio["criterio_id"]]
    assert cruda["j"] == criterio["juicio"] and cruda["e"] == criterio["evidencias"]
    guardar("recorridos.json", {
        "fallo_de_atribucion": {
            "rechazo": rechazo, "fuente": str(ruta), "linea": fuente["linea"], "pasaje": fuente["pasaje"],
            "nucleo": "backend/src/agentes/nannu/08_observabilidad_regresion/README.md",
            "control": "backend/src/agentes/nannu/entrevista.rs",
            "regresion": "backend/tests/agente_nannu.rs::localiza_fallo_de_atribucion",
            "confirmacion_real_de_la_voz": {
                "mensaje": entrevista["mensaje"], "hecho": entrevista["hecho"],
                "modelo": entrevista["invocacion"]["modelo"], "digest": entrevista["invocacion"]["digest"],
                "sha256_instrucciones": instrucciones(entrevista["invocacion"]),
            },
        },
        "criterio": {
            "entrega": uno["traza"]["entrega"], "evaluacion_id": uno["resultado"]["evaluacion_id"],
            "mensaje": uno["traza"]["mensaje"], "rubrica": uno["traza"]["rubrica"],
            "observacion": criterio, "informe": informe, "fuente": str(ruta_ecg), "linea": 4,
            "pasaje": "initial EKG (sinus tachycardia)",
            "nucleo": "backend/src/agentes/evaluador/08_observabilidad_regresion/README.md",
            "control": "backend/src/modules/evaluacion/criterios.rs",
            "regresion": "backend/tests/agente_evaluador.rs::calibra_uno_de_cuatro_y_prioriza_ecg_con_qwen",
            "salida": cruda, "modelo": invocacion["modelo"], "digest": invocacion["digest"],
            "sha256_instrucciones": instrucciones(invocacion),
            "duracion_ms": invocacion["fin_unix_ms"] - invocacion["inicio_unix_ms"],
        },
    })


if __name__ == "__main__":
    log_rust = Path(sys.argv[1]).read_text()
    matriz(log_rust)
    if len(sys.argv) > 2:
        recorridos(log_rust, Path(sys.argv[2]).read_text())
