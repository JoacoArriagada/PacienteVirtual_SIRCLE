import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

test("la compilación de ejemplo enlaza CSS y muestra su estado sin backend", async () => {
  const dist = new URL("../dist/", import.meta.url);
  const html = await readFile(new URL("ejemplos/valoracion.html", dist), "utf8");
  assert.match(html, /href="\/app\/ejemplo\.css"/);
  assert.match(html, /data-estado role="status"/);
  assert.match(html, /src="\/app\/ejemplo\.js"/);
  assert.ok((await readFile(new URL("app/ejemplo.css", dist))).length > 0);

  const script = await readFile(new URL("app/ejemplo.js", dist), "utf8");
  assert.match(script, /Interfaz de ejemplo disponible\./);
});
