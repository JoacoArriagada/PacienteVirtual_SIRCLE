import { readdir } from "node:fs/promises";
import { createServer, request } from "node:http";
import { extname } from "node:path";
import { servirArchivo, tipos } from "./estaticos.mjs";

const puertoFrontend = Number(process.env.SIRCLE_PUERTO_FRONTEND ?? 8080);
const puertoBackend = Number(process.env.SIRCLE_PUERTO_BACKEND ?? 8081);
const backend = "http://127.0.0.1:" + puertoBackend;
const archivos = new Map([["/", "index.html"]]);
for (const carpeta of ["app", "modules", "recursos"]) {
  for (const nombre of await readdir(new URL("../dist/" + carpeta, import.meta.url), { recursive: true })) {
    if (tipos.has(extname(nombre))) {
      const ruta = carpeta + "/" + nombre.replaceAll("\\", "/");
      archivos.set("/" + ruta, ruta);
    }
  }
}

function reenviar(peticion, respuesta) {
  const { host, connection, ["transfer-encoding"]: _transferencia, ...headers } = peticion.headers;
  const proxy = request(backend + peticion.url, { method: peticion.method, headers, timeout: 205000 }, (origen) => {
    const { connection, ["transfer-encoding"]: _transferencia, ...cabeceras } = origen.headers;
    respuesta.writeHead(origen.statusCode, cabeceras);
    origen.pipe(respuesta);
    origen.on("error", (error) => respuesta.destroy(error));
  });
  proxy.on("timeout", () => proxy.destroy(new Error("Axum no respondió a tiempo.")));
  proxy.on("error", (error) => {
    console.error(error.message);
    if (respuesta.headersSent) {
      respuesta.destroy(error);
    } else {
      respuesta.writeHead(502, { "Content-Type": "application/json" })
        .end(JSON.stringify({ mensaje: "Axum no está disponible. Verifica el backend local." }));
    }
  });
  peticion.on("aborted", () => proxy.destroy());
  respuesta.on("close", () => {
    if (!respuesta.writableEnded) proxy.destroy();
  });
  peticion.pipe(proxy);
}

const servidor = createServer((peticion, respuesta) => {
  const ruta = peticion.url.split("?", 1)[0];
  if (ruta === "/api" || ruta.startsWith("/api/")) {
    reenviar(peticion, respuesta);
  } else {
    void servirArchivo(peticion, respuesta, archivos.get(ruta));
  }
});
servidor.listen(puertoFrontend, "127.0.0.1", () => {
  console.log("Frontend integrado disponible en http://127.0.0.1:" + servidor.address().port);
});
