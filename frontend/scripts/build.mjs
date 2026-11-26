import { execFileSync } from "node:child_process";
import { copyFile, cp, mkdir, rm } from "node:fs/promises";

const raiz = new URL("../", import.meta.url);
const ejemplo = process.argv.includes("--ejemplo");

await rm(new URL("dist/", raiz), { recursive: true, force: true });
execFileSync(process.execPath, ["node_modules/typescript/bin/tsc", "--project", "tsconfig.json"], {
  cwd: raiz,
  stdio: "inherit",
});

await mkdir(new URL("dist/recursos/", raiz));
const archivos = [
  ["index.html", "dist/index.html"],
  ["src/app/semana5.css", "dist/app/semana5.css"],
];
if (ejemplo) {
  await mkdir(new URL("dist/ejemplos/", raiz));
  const recursos = "../docs/referencias/html-semana5/recursos/escenas/lr";
  archivos.push(
    ["ejemplos/valoracion.html", "dist/ejemplos/valoracion.html"],
    ["ejemplos/semana5.html", "dist/ejemplos/semana5.html"],
    ["src/app/ejemplo.css", "dist/app/ejemplo.css"],
    ["ejemplos/presion.json", "dist/ejemplos/presion.json"],
    [`${recursos}/presion.webp`, "dist/recursos/presion.webp"],
    [`${recursos}/presion-colocada.webp`, "dist/recursos/presion-colocada.webp"],
  );
} else {
  await rm(new URL("dist/app/ejemplo.js", raiz));
  await rm(new URL("dist/app/brazalete-ejemplo.js", raiz));
}
for (const [origen, destino] of archivos) {
  await copyFile(new URL(origen, raiz), new URL(destino, raiz));
}
await cp(new URL("recursos/semana5/", raiz), new URL("dist/recursos/semana5/", raiz), {
  recursive: true,
  filter: (ruta) => ejemplo || !/retrato-de-(?:dan-sparks|nannu)-/.test(ruta),
});
