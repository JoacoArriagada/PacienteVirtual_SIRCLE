# T070 · Origen y publicación del frontend

El 26 de noviembre de 2026 se ejecutó `docker compose -p sircle-modulo01-ensayo up --build -d` sobre la revisión `08a90308` con cambios locales. Chrome abrió la selección de identidad en `http://127.0.0.1:8080` a 1280 × 900 y escala 1. Una llamada `fetch("/api/estado")` desde esa página recibió HTTP 200 con `servicio: sircle` y SQLite disponible. [Captura](captura.png) · [Respuesta e inventario](red.json)

Los 79 archivos servidos por Nginx respondieron con HTTP 200. Sus cuerpos pasaron el cotejo con los testigos del material reservado. Las siete rutas directas de prueba para fuentes, rúbricas, aprobaciones, ejemplos, referencia y configuración devolvieron HTTP 404.
