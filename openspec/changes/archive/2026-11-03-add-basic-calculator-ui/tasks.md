# Tareas

## 1. Construir la vista estática

- [x] 1.1 Implementar en `src/interfaz/` la pantalla inicial `0` y la cuadrícula con dígitos, `+`, `−`, `×`, `÷`, `%`, `√`, decimal, borrar (`C`) e igual, sin callbacks ni estado de cálculo; verificar con `cargo check` que la vista compila.

## 2. Mostrar y revisar la interfaz

- [x] 2.1 Conectar la vista al arranque GPUI desde `src/main.rs` y retirar las impresiones de consola actuales; verificar que al iniciar se abre una ventana con la pantalla y todos los controles visibles.
- [x] 2.2 Revisar la maqueta en ejecución y pulsar los controles para verificar que no cambian la pantalla ni realizan cálculos; confirmar que los botones de porcentaje y raíz cuadrada aparecen y también permanecen inertes.

## 3. Adaptar tamaños y controles nativos

- [x] 3.1 Mantener la ventana compacta y redimensionable con tamaño mínimo, distribución flexible y escalado de texto y espacios; quitar el control propio de pantalla completa y conservar las decoraciones nativas para cerrar, minimizar y maximizar/restaurar; verificar compilación y pruebas pertinentes.
- [x] 3.2 Verificar visualmente los tamaños mínimo, compacto y grande en modo ventana, la presencia y funcionamiento de los controles nativos disponibles, la ausencia de recortes y que las teclas de cálculo sigan inertes.
