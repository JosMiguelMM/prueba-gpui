# Propuesta

## Por qué

El proyecto ya separa el módulo de interfaz GPUI de la lógica de cálculo, pero todavía no presenta una calculadora visible. Este incremento añadirá una primera vista estática para establecer la composición visual antes de implementar interacción y cálculos.

## Qué cambia

- Mostrar una pantalla de calculadora con un valor inicial de referencia `0`.
- Dibujar botones para los dígitos `0`–`9`, suma, resta, multiplicación, división, porcentaje, raíz cuadrada, separador decimal, borrar e igual.
- Organizar la pantalla por encima de un teclado en cuadrícula de disposición convencional.
- Mantener todos los botones sin acciones: no responden a pulsaciones ni realizan cálculos.
- Permitir una ventana compacta, redimensionamiento y pantalla completa, distribuyendo los elementos según el espacio disponible.
- Dejar fuera los cálculos, las acciones del teclado de la calculadora y el historial; los controles de ventana sí son interactivos.

## Capacidades

### Nuevas capacidades

- `calculator-ui`: presentación estática de una calculadora básica con pantalla y botones visibles.

### Capacidades modificadas

Ninguna.

## Impacto

- Código afectado: `src/interfaz/` para la vista y `src/main.rs` para mostrarla al iniciar la aplicación GPUI.
- Dependencias: se usarán las dependencias GPUI ya declaradas; no se propone agregar dependencias.
- Comportamiento: la aplicación mostrará una interfaz nativa en lugar de las impresiones de consola actuales; los controles serán solo visuales en este incremento.
