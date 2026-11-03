//! Analiza y evalúa expresiones aritméticas básicas sin depender de la interfaz.
//!
//! La gramática aplica la precedencia convencional: primero multiplicación y
//! división, después suma y resta, con evaluación de izquierda a derecha entre
//! operadores de la misma prioridad. Los paréntesis agrupan subexpresiones.

use crate::calculadora::error::ErrorCalculo;

/// Evalúa una expresión con números decimales, signos, paréntesis y cuatro operaciones.
///
/// Los espacios ASCII se ignoran. La multiplicación y la división tienen prioridad
/// sobre la suma y la resta; los operadores del mismo nivel se resuelven de
/// izquierda a derecha. Los signos unarios permiten escribir valores negativos.
///
/// # Ejemplos
///
/// ```
/// use prueba_gpui::calculadora::operaciones::aritmeticas::evaluar;
///
/// assert_eq!(evaluar("2 + 3 * 4"), Ok(14.0));
/// assert_eq!(evaluar("(2 + 3) * 4"), Ok(20.0));
/// ```
///
/// # Errores
///
/// Devuelve [`ErrorCalculo::SintaxisInvalida`] para entradas vacías o mal formadas,
/// [`ErrorCalculo::DivisionPorCero`] si algún divisor es cero y
/// [`ErrorCalculo::ResultadoNoFinito`] si el cálculo produce un valor no finito.
pub fn evaluar(expresion: &str) -> Result<f64, ErrorCalculo> {
    let mut analizador = Analizador::nuevo(expresion);
    let resultado = analizador.suma()?;
    analizador.ignorar_espacios();

    if analizador.posicion != analizador.entrada.len() {
        return Err(ErrorCalculo::SintaxisInvalida);
    }

    validar_resultado(resultado)
}

/// Analizador descendente que evalúa cada nivel de precedencia por separado.
struct Analizador<'a> {
    entrada: &'a [u8],
    posicion: usize,
}

impl<'a> Analizador<'a> {
    /// Crea un analizador ubicado al inicio de la entrada.
    fn nuevo(entrada: &'a str) -> Self {
        Self {
            entrada: entrada.as_bytes(),
            posicion: 0,
        }
    }

    /// Analiza sumas y restas, que tienen la menor prioridad.
    fn suma(&mut self) -> Result<f64, ErrorCalculo> {
        let mut valor = self.producto()?;
        loop {
            self.ignorar_espacios();
            match self.consumir(b'+') {
                true => valor = validar_resultado(valor + self.producto()?)?,
                false if self.consumir(b'-') => {
                    valor = validar_resultado(valor - self.producto()?)?;
                }
                false => return Ok(valor),
            }
        }
    }

    /// Analiza multiplicaciones y divisiones antes de la suma y la resta.
    fn producto(&mut self) -> Result<f64, ErrorCalculo> {
        let mut valor = self.unario()?;
        loop {
            self.ignorar_espacios();
            if self.consumir(b'*') {
                valor = validar_resultado(valor * self.unario()?)?;
            } else if self.consumir(b'/') {
                let divisor = self.unario()?;
                if divisor == 0.0 {
                    return Err(ErrorCalculo::DivisionPorCero);
                }
                valor = validar_resultado(valor / divisor)?;
            } else {
                return Ok(valor);
            }
        }
    }

    /// Analiza signos unarios, que pueden encadenarse antes de un valor.
    fn unario(&mut self) -> Result<f64, ErrorCalculo> {
        self.ignorar_espacios();
        if self.consumir(b'+') {
            self.unario()
        } else if self.consumir(b'-') {
            validar_resultado(-self.unario()?)
        } else {
            self.primario()
        }
    }

    /// Analiza un número decimal o una expresión delimitada por paréntesis.
    fn primario(&mut self) -> Result<f64, ErrorCalculo> {
        self.ignorar_espacios();
        if self.consumir(b'(') {
            let valor = self.suma()?;
            self.ignorar_espacios();
            if !self.consumir(b')') {
                return Err(ErrorCalculo::SintaxisInvalida);
            }
            return Ok(valor);
        }

        self.numero()
    }

    /// Lee un número decimal con punto y al menos un dígito.
    fn numero(&mut self) -> Result<f64, ErrorCalculo> {
        self.ignorar_espacios();
        let inicio = self.posicion;
        let mut tiene_digitos = self.consumir_digitos();

        if self.consumir(b'.') {
            tiene_digitos |= self.consumir_digitos();
        }
        if !tiene_digitos {
            return Err(ErrorCalculo::SintaxisInvalida);
        }

        let texto = std::str::from_utf8(&self.entrada[inicio..self.posicion])
            .map_err(|_| ErrorCalculo::SintaxisInvalida)?;
        let valor = texto
            .parse::<f64>()
            .map_err(|_| ErrorCalculo::SintaxisInvalida)?;
        validar_resultado(valor)
    }

    /// Consume una secuencia de dígitos ASCII e indica si había alguno.
    fn consumir_digitos(&mut self) -> bool {
        let inicio = self.posicion;
        while self
            .actual()
            .is_some_and(|caracter| caracter.is_ascii_digit())
        {
            self.posicion += 1;
        }
        self.posicion > inicio
    }

    /// Consume el carácter indicado si aparece en la posición actual.
    fn consumir(&mut self, esperado: u8) -> bool {
        if self.actual() == Some(esperado) {
            self.posicion += 1;
            true
        } else {
            false
        }
    }

    /// Omite espacios ASCII entre los componentes de la expresión.
    fn ignorar_espacios(&mut self) {
        while self
            .actual()
            .is_some_and(|caracter| caracter.is_ascii_whitespace())
        {
            self.posicion += 1;
        }
    }

    /// Devuelve el byte actual de la entrada, si aún quedan caracteres.
    fn actual(&self) -> Option<u8> {
        self.entrada.get(self.posicion).copied()
    }
}

/// Rechaza resultados infinitos o NaN para mantener los errores explícitos.
fn validar_resultado(valor: f64) -> Result<f64, ErrorCalculo> {
    if valor.is_finite() {
        Ok(valor)
    } else {
        Err(ErrorCalculo::ResultadoNoFinito)
    }
}

#[cfg(test)]
mod pruebas {
    use super::evaluar;
    use crate::calculadora::error::ErrorCalculo;

    /// Comprueba la precedencia y la asociatividad izquierda de las operaciones.
    #[test]
    fn respeta_precedencia_y_asociatividad() {
        for (expresion, esperado) in [
            ("2 + 3 * 4", 14.0),
            ("(2 + 3) * 4", 20.0),
            ("8 / 4 * 2", 4.0),
            ("10 - 3 - 2", 5.0),
            ("20 / 5 / 2", 2.0),
        ] {
            assert_eq!(evaluar(expresion), Ok(esperado), "{expresion}");
        }
    }

    /// Comprueba números decimales y signos unarios.
    #[test]
    fn acepta_decimales_y_signos_unarios() {
        for (expresion, esperado) in [
            ("2.5 + .5", 3.0),
            ("-3 + 5", 2.0),
            ("-(2 + 3) * -2", 10.0),
            ("+4", 4.0),
        ] {
            assert_eq!(evaluar(expresion), Ok(esperado), "{expresion}");
        }
    }

    /// Comprueba que las entradas vacías o con sintaxis incorrecta se rechazan.
    #[test]
    fn rechaza_expresiones_mal_formadas() {
        for expresion in ["", "   ", "2 +", "()", "(2 + 3", "2 + 3)", "2..1", "2 3"] {
            assert_eq!(
                evaluar(expresion),
                Err(ErrorCalculo::SintaxisInvalida),
                "{expresion:?}"
            );
        }
    }

    /// Comprueba la división directa e indirecta entre cero.
    #[test]
    fn detecta_division_entre_cero() {
        for expresion in ["1 / 0", "1 / (2 - 2)", "1 / -0.0"] {
            assert_eq!(
                evaluar(expresion),
                Err(ErrorCalculo::DivisionPorCero),
                "{expresion}"
            );
        }
    }

    /// Comprueba que los cálculos no devuelven valores infinitos ni NaN.
    #[test]
    fn rechaza_resultados_no_finitos() {
        assert_eq!(
            evaluar("1e308 * 1e308"),
            Err(ErrorCalculo::SintaxisInvalida)
        );
        let numero_grande = "9".repeat(200);
        let expresion = format!("{numero_grande} * {numero_grande}");
        assert_eq!(evaluar(&expresion), Err(ErrorCalculo::ResultadoNoFinito));
    }
}
