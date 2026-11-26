import { readdir } from "node:fs/promises";
import { createServer } from "node:http";
import { servirArchivo } from "./estaticos.mjs";

const archivos = new Map([
  ["/", "ejemplos/valoracion.html"],
  ["/semana5", "ejemplos/semana5.html"],
  ["/aplicacion", "index.html"],
  ["/ejemplos/presion.json", "ejemplos/presion.json"],
  ["/recursos/presion.webp", "recursos/presion.webp"],
  ["/recursos/presion-colocada.webp", "recursos/presion-colocada.webp"],
]);
for (const carpeta of ["app", "modules", "recursos/semana5"]) {
  for (const nombre of await readdir(new URL("../dist/" + carpeta, import.meta.url), { recursive: true })) {
    const ruta = carpeta + "/" + nombre.replaceAll("\\", "/");
    archivos.set("/" + ruta, ruta);
  }
}
const servidor = createServer((peticion, respuesta) => {
  void servirArchivo(peticion, respuesta, archivos.get(peticion.url.split("?", 1)[0]));
});
servidor.listen(4173, "127.0.0.1", () => {
  console.log("Frontend de ejemplo disponible en http://127.0.0.1:4173");
});
