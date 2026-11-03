# Proposal

## Why

La calculadora presenta las cuatro operaciones aritméticas básicas, pero sus controles todavía no responden y no existe una lógica que evalúe expresiones. Activar la entrada y el cálculo con la precedencia matemática habitual permitirá resolver expresiones compuestas de manera predecible, incluidos los casos agrupados con paréntesis.

## What Changes

- Activar la entrada de dígitos y decimales, las operaciones suma, resta, multiplicación y división, el borrado, los paréntesis y la evaluación mediante igual.
- Evaluar multiplicación y división antes que suma y resta; resolver operadores de igual prioridad de izquierda a derecha y respetar los paréntesis.
- Mostrar resultados y comunicar entradas inválidas y división entre cero sin bloquear la aplicación.
- Añadir las teclas de paréntesis a la interfaz.
- Mantener porcentaje y raíz cuadrada visibles pero fuera de la funcionalidad de este cambio.

## Capabilities

### New Capabilities

<!-- No se crea una capacidad paralela: el comportamiento pertenece a la calculadora existente. -->

### Modified Capabilities

- `calculator-ui`: sustituir el requisito de botones inactivos por interacción y evaluación de expresiones con las cuatro operaciones básicas y paréntesis.

## Impact

- Código afectado: `src/interfaz/mod.rs` para el estado e interacción de la interfaz; `src/calculadora/operaciones/aritmeticas.rs` y los módulos de `src/calculadora/` para el análisis, cálculo y errores.
- Se actualizarán las pruebas y la documentación Rustdoc en español de la lógica afectada.
- No se prevén dependencias nuevas ni cambios de API externa; se prioriza una solución idiomática en Rust.
- Criterios de éxito: `2 + 3 × 4` produce `14`; `(2 + 3) × 4` produce `20`; `8 ÷ 4 × 2` produce `4`; división entre cero y expresiones mal formadas se manejan como errores recuperables.
