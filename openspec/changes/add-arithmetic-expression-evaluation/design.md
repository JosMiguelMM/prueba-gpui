# Design

## Context

La interfaz de `src/interfaz/mod.rs` es actualmente una vista GPUI sin estado, y los módulos de `src/calculadora/operaciones/` no contienen lógica de cálculo. La especificación de `calculator-ui` define los controles y la ventana existentes; este cambio actualiza el comportamiento de los botones, sin alterar el framework ni añadir dependencias.

## Goals / Non-Goals

**Goals:**

- Separar el análisis y cálculo de expresiones de la capa GPUI, de modo que la lógica se pruebe como Rust ordinario.
- Admitir números decimales, suma, resta, multiplicación, división y paréntesis con precedencia convencional.
- Representar fallos de sintaxis y división entre cero como errores recuperables que la interfaz pueda mostrar.

**Non-Goals:**

- Implementar porcentaje, raíz cuadrada, teclado físico, memoria u otras funciones de calculadora.
- Incorporar un motor de expresiones externo o crear una abstracción general para operaciones futuras.

## Decisions

1. **Analizador recursivo descendente, no expresiones regulares.** La gramática distingue niveles de precedencia: expresión/suma, producto, unario y valor primario. La suma y la resta consumen productos; el producto consume factores con multiplicación y división; los paréntesis vuelven a analizar una expresión completa. Esto hace explícita la precedencia y el agrupamiento, que no se pueden resolver con una regex general de forma segura. Se considerará un número decimal como un primario y se admitirá signo unario para expresar operandos negativos.
   - Alternativa considerada: evaluar de izquierda a derecha al pulsar cada operador. Se descarta porque no respeta precedencia en expresiones como `2 + 3 × 4`.
   - Alternativa considerada: agregar una biblioteca de evaluación. Se descarta por el alcance pequeño y para evitar dependencia nueva.

2. **Lógica matemática pura y errores tipados.** El módulo aritmético evaluará la expresión sin depender de GPUI. Los errores distinguirán al menos sintaxis inválida y división entre cero; la interfaz traducirá esos errores a un mensaje visible y permitirá borrar para recuperarse. La división se comprobará antes de operar, evitando resultados infinitos o NaN por divisor cero.

3. **La interfaz mantiene la expresión como estado editable.** Los botones de alcance modificarán la entrada; igual solicitará la evaluación, y borrar restablecerá el estado inicial. Los signos visuales `×`, `÷` y `−` se traducirán a los operadores reconocidos por la lógica. Se agregarán controles de paréntesis y se ajustará la cuadrícula para que la ventana siga mostrando los controles.

4. **Sin migración ni dependencias externas.** No hay datos persistidos ni contratos externos que migrar. La solución se implementará con Rust y GPUI ya usados por el proyecto.

## Risks / Trade-offs

- [Una expresión parcial o paréntesis desbalanceados puede fallar al evaluar] → El analizador devolverá error, no entrará en pánico, y la interfaz mostrará una indicación que permita corregir o borrar.
- [El formato de punto flotante puede mostrar representaciones largas] → Formatear el resultado para la pantalla sin cambiar el cálculo interno; probar resultados enteros y decimales representativos.
- [Agregar controles puede comprimir el teclado] → Reorganizar las filas y comprobar la ventana mínima y las pruebas visuales o de interacción disponibles.

## Migration Plan

No requiere migración de datos. Se reemplaza gradualmente la vista sin acciones por controles con estado e integración con el evaluador; si la integración causa problemas, el cambio puede revertirse sin afectar persistencia ni interfaces externas.

## Open Questions

Ninguna que bloquee el diseño. El formato exacto de presentación de decimales puede fijarse durante la implementación siempre que conserve el valor calculado y sea legible.
