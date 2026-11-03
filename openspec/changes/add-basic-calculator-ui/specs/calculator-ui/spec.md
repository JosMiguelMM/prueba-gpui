# Spec Delta

## MODIFIED Requirements

### Requirement: La interfaz muestra una pantalla y un teclado básico
La interfaz SHALL mostrar una pantalla con el valor inicial `0` y botones visibles para los dígitos del 0 al 9, suma, resta, multiplicación, división, porcentaje, raíz cuadrada, separador decimal, borrar e igual.

#### Scenario: Se muestra la calculadora al abrir la aplicación
- **WHEN** se inicia la aplicación
- **THEN** la interfaz presenta la pantalla con `0` y todos los botones requeridos

### Requirement: Los botones son únicamente visuales
Los botones SHALL permanecer sin acciones asociadas: pulsarlos no cambiará la pantalla ni ejecutará cálculos.

#### Scenario: Se pulsa un botón del teclado
- **WHEN** se pulsa cualquier botón de dígito, operación, decimal, borrar, porcentaje, raíz cuadrada o igual
- **THEN** no se produce ningún cambio en la pantalla ni se realiza una operación
