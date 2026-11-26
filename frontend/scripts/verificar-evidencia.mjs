import { spawnSync } from "node:child_process";

const [modo, destino, base] = process.argv.slice(2);
const configuracion = {
  paridad: { prefijo: "SIRCLE_PARIDAD", prueba: "paridad_semana5.test.ts" },
  reserva: { prefijo: "SIRCLE_RESERVA", prueba: "reserva_publica.test.ts" },
}[modo];

if (!configuracion) throw new Error("Selecciona la prueba de paridad o reserva.");

const { prefijo, prueba } = configuracion;
const env = { ...process.env };
if (destino) env[prefijo + "_EVIDENCIA"] = destino;
if (base) env[prefijo + "_DB"] = base;
if (!env[prefijo + "_DB"]) throw new Error("Indica la base de ensayo como segundo argumento.");

const resultado = spawnSync(process.execPath, ["--test", "tests/" + prueba], { env, stdio: "inherit" });
if (resultado.error) throw resultado.error;
process.exitCode = resultado.status ?? 1;
