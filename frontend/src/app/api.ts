export class ErrorHttp extends Error {
  constructor(readonly estado: number, mensaje: string) {
    super(mensaje);
  }
}

export async function api<T>(ruta: string, opciones?: RequestInit): Promise<T> {
  const respuesta = await fetch("/api" + ruta, { cache: "no-store", ...opciones });
  if (respuesta.status === 202) {
    throw new ErrorHttp(202, "La solicitud sigue en curso. Vuelve a enviarla para consultar la confirmación.");
  }
  if (!respuesta.ok) {
    const error: { mensaje?: string } = await respuesta.json().catch(() => ({}));
    throw new ErrorHttp(respuesta.status, error.mensaje ?? "No se pudo completar la solicitud.");
  }
  return respuesta.status === 204 ? undefined as T : await respuesta.json() as T;
}

export function escribir(metodo: string, datos: object, clave?: string): RequestInit {
  const headers = new Headers({ "Content-Type": "application/json" });
  if (clave) headers.set("Idempotency-Key", clave);
  return { method: metodo, headers, body: JSON.stringify(datos) };
}
