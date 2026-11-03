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

## ADDED Requirements

### Requirement: La calculadora adapta sus elementos al tamaño de la ventana
La interfaz SHALL abrir en una ventana compacta, permitir redimensionarla y distribuir pantalla, teclado, texto y separaciones según el espacio disponible. SHALL mantener cuatro columnas y el cero de doble ancho, sin recortes, solapamientos ni desplazamiento desde el tamaño mínimo admitido.

#### Scenario: Se amplía o reduce la ventana
- **WHEN** se cambia el tamaño entre el mínimo admitido y una ventana grande
- **THEN** los elementos adaptan su tamaño y aprovechan el espacio disponible manteniendo todos los controles visibles y legibles

### Requirement: La ventana permite alternar pantalla completa
La aplicación SHALL ofrecer un control de ventana para entrar y salir de pantalla completa, independiente de los botones inertes de la calculadora.

#### Scenario: Se entra y sale de pantalla completa
- **WHEN** se activa el control de pantalla completa y después se vuelve a activar
- **THEN** la calculadora ocupa primero la pantalla disponible y después vuelve al tamaño anterior de ventana, reajustando sus elementos en ambos casos
