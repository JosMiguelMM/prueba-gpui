# Spec Delta

## MODIFIED Requirements

### Requirement: La interfaz muestra una pantalla y un teclado básico
La interfaz SHALL mostrar una pantalla con el valor inicial `0` y botones visibles para los dígitos del 0 al 9, suma, resta, multiplicación, división, porcentaje, raíz cuadrada, separador decimal, borrar e igual. SHALL ofrecer también controles para abrir y cerrar paréntesis, manteniendo todos los controles visibles.

#### Scenario: Se muestra la calculadora al abrir la aplicación
- **WHEN** se inicia la aplicación
- **THEN** la interfaz presenta la pantalla con `0`, los controles existentes y los controles de paréntesis

### Requirement: Los botones son únicamente visuales
Los botones de dígitos, separador decimal, suma, resta, multiplicación, división, paréntesis, borrar e igual SHALL permitir construir, corregir y evaluar expresiones. La multiplicación y la división SHALL tener prioridad sobre la suma y la resta; operadores de igual prioridad SHALL evaluarse de izquierda a derecha y los paréntesis SHALL agrupar subexpresiones. Los botones de porcentaje y raíz cuadrada SHALL permanecer sin acciones de cálculo.

#### Scenario: Se pulsa un botón del teclado
- **WHEN** se pulsa un botón de dígito, operación básica, decimal, paréntesis, borrar, porcentaje, raíz cuadrada o igual
- **THEN** se actualiza la entrada si el control está dentro del alcance; porcentaje y raíz cuadrada no ejecutan cálculos

#### Scenario: Se evalúa primero la multiplicación
- **WHEN** el usuario introduce `2 + 3 × 4` y pulsa igual
- **THEN** la pantalla muestra `14`

#### Scenario: Los paréntesis alteran el orden de evaluación
- **WHEN** el usuario introduce `(2 + 3) × 4` y pulsa igual
- **THEN** la pantalla muestra `20`

#### Scenario: Operadores con la misma prioridad se evalúan de izquierda a derecha
- **WHEN** el usuario introduce `8 ÷ 4 × 2` y pulsa igual
- **THEN** la pantalla muestra `4`

#### Scenario: La resta conserva el orden de izquierda a derecha
- **WHEN** el usuario introduce `10 − 3 − 2` y pulsa igual
- **THEN** la pantalla muestra `5`

#### Scenario: Se puede borrar la entrada
- **WHEN** el usuario pulsa borrar después de introducir una expresión
- **THEN** la entrada se limpia y la pantalla vuelve a mostrar `0`

#### Scenario: Se informa la división entre cero
- **WHEN** el usuario evalúa una expresión cuyo divisor es cero
- **THEN** la pantalla muestra un error comprensible y la aplicación permanece disponible para una nueva entrada

#### Scenario: Se informa una expresión mal formada
- **WHEN** el usuario evalúa una expresión incompleta o con paréntesis sin correspondencia
- **THEN** la aplicación informa el error sin bloquearse
