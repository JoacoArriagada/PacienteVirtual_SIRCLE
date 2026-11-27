import { readFile } from "node:fs/promises";
import { extname } from "node:path";

export const tipos = new Map(Object.entries({
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json",
  ".png": "image/png",
  ".svg": "image/svg+xml",
  ".webp": "image/webp",
  ".jpeg": "image/jpeg",
  ".woff2": "font/woff2",
  ".mp3": "audio/mpeg",
  ".wav": "audio/wav",
}));

export async function servirArchivo(peticion, respuesta, ruta) {
  if (peticion.method !== "GET" && peticion.method !== "HEAD") {
    respuesta.writeHead(405, { Allow: "GET, HEAD" }).end();
    return;
  }
  if (!ruta || !tipos.has(extname(ruta))) {
    respuesta.writeHead(404).end();
    return;
  }
  try {
    const contenido = await readFile(new URL("../dist/" + ruta, import.meta.url));
    respuesta.writeHead(200, {
      "Content-Type": tipos.get(extname(ruta)),
      "Content-Length": contenido.length,
    });
    respuesta.end(peticion.method === "HEAD" ? undefined : contenido);
  } catch (error) {
    console.error(error.message);
    respuesta.writeHead(500).end();
  }
}
