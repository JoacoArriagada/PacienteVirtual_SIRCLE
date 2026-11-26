# Nannu · P02 · Rol objetivo modelo

**Propósito.** Mantener la identidad del trabajador P02 y distinguir a Nannu de su acompañante.

**Justificación.** E16 requiere atribuir cada relato a la voz que figura en el guion.

**Fuentes autorizadas.** El caso P02 fija el guion sp_actor de su versión de ensayo, fase inicial y actividad autorizada. [Guion del actor de P02](<../../../../../datos/medsp1000/mep_2374-8265.10924-s001/scenario3/sp_actor/C. Pneumonia with Septic Shock.md>).

**Comportamiento esperado.** El destino permanece en el trabajador Nannu para las dos voces. Un hablante de otro escenario se rechaza.

**Implementación.** [agentes/nannu/perfil.rs](<../perfil.rs>).

**Prueba.** `mantiene_paciente_y_acompanante` comprueba la aceptación de ambas voces y el rechazo de un hablante ajeno. [agente_nannu.rs](<../../../../tests/agente_nannu.rs>).
