//! Define los errores recuperables que puede producir la lógica de cálculo.

use std::fmt;

/// Describe por qué no se pudo evaluar una expresión aritmética.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCalculo {
    /// La entrada está vacía o no respeta la sintaxis aritmética admitida.
    SintaxisInvalida,
    /// Se intentó dividir un valor entre cero.
    DivisionPorCero,
    /// Una operación produjo infinito o NaN.
    ResultadoNoFinito,
}

impl fmt::Display for ErrorCalculo {
    /// Proporciona un mensaje breve en español para cada error de cálculo.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SintaxisInvalida => f.write_str("La expresión no es válida"),
            Self::DivisionPorCero => f.write_str("No se puede dividir entre cero"),
            Self::ResultadoNoFinito => f.write_str("El resultado no es un número finito"),
        }
    }
}

impl std::error::Error for ErrorCalculo {}
