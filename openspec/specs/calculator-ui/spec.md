# calculator-ui Specification

## Purpose

Presenta una calculadora básica en una ventana nativa para establecer la interfaz visual antes de añadir interacciones y cálculos.

## Requirements

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

### Requirement: La calculadora adapta sus elementos al tamaño de la ventana
La interfaz SHALL abrir en una ventana compacta, permitir redimensionarla y distribuir pantalla, teclado, texto y separaciones según el espacio disponible. SHALL mantener cuatro columnas y el cero de doble ancho, sin recortes, solapamientos ni desplazamiento desde el tamaño mínimo admitido.

#### Scenario: Se amplía o reduce la ventana
- **WHEN** se cambia el tamaño entre el mínimo admitido y una ventana grande
- **THEN** los elementos adaptan su tamaño y aprovechan el espacio disponible manteniendo todos los controles visibles y legibles

### Requirement: La ventana usa los controles nativos del sistema
La aplicación SHALL conservar las decoraciones nativas de su ventana y no dibujar controles propios para cerrar, minimizar, maximizar o entrar en pantalla completa. La disponibilidad y el comportamiento de estos controles SHALL seguir las convenciones del sistema operativo y del gestor de ventanas.

#### Scenario: Se abre la calculadora en una ventana normal
- **WHEN** se inicia la aplicación en modo ventana
- **THEN** el sistema muestra su barra de título y los controles nativos disponibles para cerrar, minimizar y maximizar o restaurar la ventana

#### Scenario: Se usa un control nativo de ventana
- **WHEN** el usuario activa un control nativo disponible
- **THEN** el sistema cierra, minimiza o maximiza/restaura la ventana según el control y sus convenciones
