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
await copyFile(new URL("index.html", raiz), new URL("dist/index.html", raiz));
await copyFile(new URL("src/app/styles.css", raiz), new URL("dist/app/styles.css", raiz));

if (ejemplo) {
  await mkdir(new URL("dist/demo/", raiz));
  await copyFile(new URL("demo/simulacion.html", raiz), new URL("dist/demo/simulacion.html", raiz));
}

await cp(new URL("recursos/simulacion/", raiz), new URL("dist/recursos/simulacion/", raiz), {
  recursive: true,
  filter: (ruta) => ejemplo || !/retrato-de-(?:dan-sparks|nannu)-/.test(ruta),
});
