# Tareas

## 1. Construir la vista estática

- [x] 1.1 Implementar en `src/interfaz/` la pantalla inicial `0` y la cuadrícula con dígitos, `+`, `−`, `×`, `÷`, `%`, `√`, decimal, borrar (`C`) e igual, sin callbacks ni estado de cálculo; verificar con `cargo check` que la vista compila.

## 2. Mostrar y revisar la interfaz

- [ ] 2.1 Conectar la vista al arranque GPUI desde `src/main.rs` y retirar las impresiones de consola actuales; verificar que al iniciar se abre una ventana con la pantalla y todos los controles visibles.
- [ ] 2.2 Revisar la maqueta en ejecución y pulsar los controles para verificar que no cambian la pantalla ni realizan cálculos; confirmar que los botones de porcentaje y raíz cuadrada aparecen y también permanecen inertes.

## 3. Adaptar tamaños y pantalla completa

- [x] 3.1 Implementar ventana compacta con tamaño mínimo, distribución flexible y escalado de texto y espacios; agregar un control independiente para alternar pantalla completa y verificar compilación y pruebas pertinentes.
- [ ] 3.2 Verificar visualmente ventana mínima, compacta, grande y pantalla completa, incluyendo restauración del tamaño anterior, ausencia de recortes y botones de cálculo inertes.
