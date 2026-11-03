# Diseño

## Contexto

Consulta `proposal.md` para la motivación y `specs/calculator-ui/spec.md` para el comportamiento observable. `src/main.rs` aún imprime texto y `src/interfaz/mod.rs` solo declara el límite del módulo; GPUI y `gpui_platform` ya están declarados como dependencias.

## Objetivos / Fuera de alcance

**Objetivos:**
- Construir una vista de calculadora sencilla dentro de `interfaz` y mostrarla al iniciar la aplicación GPUI.
- Usar componentes visuales estáticos, sin estado ni callbacks de interacción.

**Fuera de alcance:**
- Modificar o invocar la lógica de `calculadora`.
- Añadir cálculos, callbacks, captura de pulsaciones, accesibilidad de teclado, historial o estilos de marca; porcentaje y raíz cuadrada se presentan únicamente como botones visuales.

## Decisiones

### Mantener toda la presentación en el módulo `interfaz`

La vista y sus elementos visuales pertenecen a `src/interfaz/`; `main.rs` se limita a iniciar GPUI y mostrar esa vista. La lógica matemática queda intacta y no forma parte del flujo de renderizado en este incremento.

**Alternativa considerada:** construir controles en `main.rs`. Se descarta porque mezclaría el punto de entrada con la presentación y desharía la separación de módulos existente.

### Usar una composición estática y convencional

La pantalla ocupa la parte superior y muestra `0`. Debajo, los controles se organizan en cuatro columnas: primera fila `C`, `√`, `%`, `÷`; luego las filas `7 8 9 ×`, `4 5 6 −` y `1 2 3 +`; la última fila contiene `0` ocupando dos columnas, `.`, e `=`. Los elementos tendrán apariencia de botones, pero no tendrán manejadores ni cambiarán la pantalla. Los botones `√` y `%` no ejecutan sus operaciones.

Se elige esta composición común para que la interfaz sea reconocible sin introducir decisiones de diseño visual más amplias. No se fija una paleta, tipografía ni tema de marca.

**Alternativa considerada:** crear un sistema de estilos reutilizable o controles interactivos. Se descarta por ser innecesario para una maqueta estática.

### Reutilizar GPUI sin agregar dependencias

La implementación utilizará las capacidades de presentación ya disponibles en GPUI. No se introducen crates adicionales ni se alteran versiones del manifiesto.

## Riesgos / Compromisos

- [La aplicación pasará de imprimir texto a iniciar una ventana nativa] → Verificar durante la implementación que la ventana muestra la vista y que el binario sigue compilando.
- [Una maqueta sin callbacks no demuestra comportamiento de cálculo] → Mantener explícito el carácter visual de este incremento y dejar la interacción para un cambio posterior.
- [El orden exacto de los controles puede requerir ajustes por el tamaño de la ventana] → Usar una cuadrícula flexible y comprobar que todos los controles permanecen visibles en la ventana inicial.
