import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";
import { Brazalete } from "../dist/modules/valoracion/brazalete.js";

test("el brazalete exige colocación y conserva una sola referencia al registrar otra vez", async () => {
  const lectura = JSON.parse(await readFile(new URL("../dist/ejemplos/presion.json", import.meta.url), "utf8"));
  const brazalete = new Brazalete();
  const cargar = async () => lectura;

  brazalete.colocar();
  await brazalete.tomar(cargar);
  assert.equal(brazalete.etapa, "inicio");
  assert.equal(brazalete.lectura, null);

  brazalete.preparar();
  assert.equal(brazalete.etapa, "preparado");
  brazalete.colocar();
  assert.equal(brazalete.etapa, "colocado");
  await brazalete.tomar(cargar);
  assert.equal(brazalete.etapa, "resultado");
  assert.deepEqual(brazalete.lectura, lectura);

  brazalete.registrar();
  brazalete.registrar();
  assert.deepEqual([...brazalete.historico.entries()], [[lectura.medicion_id, lectura]]);
});
