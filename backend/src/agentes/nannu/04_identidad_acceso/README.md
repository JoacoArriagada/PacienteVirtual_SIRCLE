# Nannu · P02 · Identidad acceso

**Propósito.** Verificar el propietario y la asignación antes de cambiar el destinatario.

**Justificación.** E16 y E17 conservan los datos de la sesión al alternar voces.

**Fuentes autorizadas.** Sesión, propietario, actividad P02 habilitada para ensayo y fase inicial. El intento mantiene su versión. [Guion del actor de P02](<../../../../../datos/medsp1000/mep_2374-8265.10924-s001/scenario3/sp_actor/C. Pneumonia with Septic Shock.md>).

**Comportamiento esperado.** El cambio de voz conserva intento, estudiante, versión y fase. El estudiante ajeno recibe 403 antes de recuperar contexto.

**Implementación.** [shared/autorizacion.rs](<../../../shared/autorizacion.rs>).

**Prueba.** `cambiar_hablante_conserva_propietario` comprueba la identidad compartida y el rechazo del intento ajeno. [agente_nannu.rs](<../../../../tests/agente_nannu.rs>).
