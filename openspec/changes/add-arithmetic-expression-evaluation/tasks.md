# Tasks

## 1. Evaluador de expresiones

- [x] 1.1 Implementar en la lógica aritmética un analizador recursivo descendente para números decimales, signos unarios, paréntesis y `+`, `-`, `*`, `/`, con precedencia y asociatividad requeridas; documentar módulos, tipos y funciones en Rustdoc español y verificar con pruebas unitarias casos como `2 + 3 * 4 = 14`, `(2 + 3) * 4 = 20`, `8 / 4 * 2 = 4`, `10 - 3 - 2 = 5`, negativos y decimales.
- [x] 1.2 Definir errores tipados para sintaxis inválida y división entre cero, con mensajes recuperables y documentación Rustdoc; verificar mediante pruebas expresiones incompletas, paréntesis desbalanceados y división por cero, sin pánicos.

## 2. Interfaz interactiva

- [x] 2.1 Añadir estado de entrada y acciones a los controles de dígitos, decimal, cuatro operaciones, paréntesis, borrar e igual; actualizar la distribución visual para incluir paréntesis y comprobar que los controles siguen visibles desde el tamaño mínimo.
- [x] 2.2 Conectar la evaluación a la pantalla, presentar el resultado o un error comprensible y permitir borrar para iniciar una nueva entrada; verificar manualmente o con pruebas disponibles los flujos de precedencia, paréntesis, resultado, error y recuperación.

## 3. Integración y validación

- [x] 3.1 Ejecutar `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo test --doc` y `cargo doc --no-deps`; resolver fallos introducidos por el cambio y registrar resultados.
- [ ] 3.2 Ejecutar la aplicación y comprobar que las cuatro operaciones y los paréntesis funcionan de extremo a extremo, que la división entre cero no bloquea la interfaz y que porcentaje y raíz cuadrada continúan fuera del alcance de este cambio.
