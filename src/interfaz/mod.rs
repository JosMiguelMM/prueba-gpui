//! Presentación e interacción de la calculadora mediante GPUI.

use gpui::{
    App, Bounds, ClickEvent, Context, InteractiveElement, StatefulInteractiveElement,
    TitlebarOptions, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use gpui_platform::application;

use crate::calculadora::operaciones::aritmeticas::evaluar;

/// Vista interactiva de la calculadora aritmética básica.
#[derive(Default)]
pub struct Calculadora {
    /// Expresión que el usuario está construyendo o que acaba de evaluar.
    entrada: String,
    /// Resultado más reciente, mostrado hasta que se introduzca otro valor.
    resultado: Option<String>,
    /// Mensaje de error más reciente, recuperable mediante una nueva entrada.
    error: Option<String>,
}

impl Render for Calculadora {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let escala = escala_visual(
            f32::from(window.viewport_size().width),
            f32::from(window.viewport_size().height),
        );
        let pantalla = self.texto_pantalla().to_owned();
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap(px(8. * escala))
            .p(px(16. * escala))
            .bg(rgb(0x202124))
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .w_full()
                    .grid()
                    .grid_cols(4)
                    .grid_rows(7)
                    .gap(px(8. * escala))
                    .child(
                        div()
                            .col_span_full()
                            .w_full()
                            .h_full()
                            .min_h_0()
                            .min_w_0()
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_lg()
                            .bg(rgb(0x111214))
                            .text_size(px(32. * escala))
                            .text_color(rgb(0xffffff))
                            .child(pantalla),
                    )
                    .child(tecla(
                        "C",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.limpiar();
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        "(",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("(");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        ")",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar(")");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        "÷",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("÷");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        "7",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("7");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        "8",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("8");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        "9",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("9");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        "×",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("×");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        "4",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("4");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        "5",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("5");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        "6",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("6");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        "−",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("−");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        "1",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("1");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        "2",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("2");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        "3",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("3");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        "+",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("+");
                            cx.notify();
                        }),
                    ))
                    .child(tecla_inactiva("√", 1, escala))
                    .child(tecla(
                        "0",
                        2,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar("0");
                            cx.notify();
                        }),
                    ))
                    .child(tecla(
                        ".",
                        1,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.insertar(".");
                            cx.notify();
                        }),
                    ))
                    .child(tecla_inactiva("%", 1, escala))
                    .child(tecla(
                        "=",
                        3,
                        escala,
                        cx.listener(|this, _, _, cx| {
                            this.calcular();
                            cx.notify();
                        }),
                    )),
            )
    }
}

impl Calculadora {
    /// Devuelve el texto que debe aparecer en la pantalla.
    fn texto_pantalla(&self) -> &str {
        self.error
            .as_deref()
            .or(self.resultado.as_deref())
            .unwrap_or_else(|| {
                if self.entrada.is_empty() {
                    "0"
                } else {
                    &self.entrada
                }
            })
    }

    /// Incorpora un dígito, operador o paréntesis a la expresión actual.
    fn insertar(&mut self, token: &str) {
        if self.error.take().is_some() {
            self.entrada.clear();
            self.resultado = None;
        }
        if let Some(resultado) = self.resultado.take() {
            if matches!(token, "+" | "−" | "×" | "÷") {
                self.entrada = resultado;
            } else {
                self.entrada.clear();
            }
        }
        self.entrada.push_str(token);
    }

    /// Evalúa la expresión actual y guarda el resultado o un mensaje recuperable.
    fn calcular(&mut self) {
        let expresion: String = self
            .entrada
            .chars()
            .map(|caracter| match caracter {
                '×' => '*',
                '÷' => '/',
                '−' => '-',
                otro => otro,
            })
            .collect();

        match evaluar(&expresion) {
            Ok(resultado) => {
                self.resultado = Some(resultado.to_string());
                self.error = None;
            }
            Err(error) => {
                self.resultado = None;
                self.error = Some(error.to_string());
            }
        }
    }

    /// Restablece la pantalla y elimina errores y resultados previos.
    fn limpiar(&mut self) {
        self.entrada.clear();
        self.resultado = None;
        self.error = None;
    }
}

/// Inicia GPUI y abre una ventana centrada con la vista de la calculadora.
pub fn iniciar() {
    application().run(|cx: &mut App| {
        let bounds = WindowBounds::Windowed(Bounds::centered(None, size(px(360.), px(520.)), cx));
        let resultado = cx.open_window(
            WindowOptions {
                window_bounds: Some(bounds),
                window_min_size: Some(size(px(280.), px(400.))),
                titlebar: Some(TitlebarOptions {
                    title: Some("Calculadora".into()),
                    ..Default::default()
                }),
                is_resizable: true,
                is_minimizable: true,
                ..Default::default()
            },
            |_, cx| cx.new(|_| Calculadora::default()),
        );
        if let Err(error) = resultado {
            eprintln!("No se pudo abrir la ventana de la calculadora: {error}");
            cx.quit();
            return;
        }
        cx.activate(true);
    });
}

/// Construye una tecla interactiva con el estilo común de la calculadora.
fn tecla(
    etiqueta: &'static str,
    columnas: u16,
    escala: f32,
    accion: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    estilo_tecla(etiqueta, columnas, escala)
        .id(etiqueta)
        .on_click(accion)
}

/// Construye un control visible que todavía no ejecuta una operación.
fn tecla_inactiva(etiqueta: &'static str, columnas: u16, escala: f32) -> impl IntoElement {
    estilo_tecla(etiqueta, columnas, escala)
}

/// Aplica el estilo compartido de las teclas.
fn estilo_tecla(etiqueta: &'static str, columnas: u16, escala: f32) -> gpui::Div {
    div()
        .when(columnas > 1, |elemento| elemento.col_span(columnas))
        .h_full()
        .min_h_0()
        .min_w_0()
        .flex()
        .items_center()
        .justify_center()
        .rounded_md()
        .border_1()
        .border_color(rgb(0x45474a))
        .bg(rgb(0x303236))
        .text_size(px(24. * escala))
        .text_color(rgb(0xffffff))
        .child(etiqueta)
}

/// Escala la tipografía y las separaciones según la dimensión limitante.
///
/// El mínimo mantiene legibilidad en la ventana mínima; el máximo evita
/// textos desproporcionados en pantallas grandes sin limitar la cuadrícula.
fn escala_visual(ancho: f32, alto: f32) -> f32 {
    (ancho / 360.).min(alto / 520.).clamp(0.75, 3.)
}

#[cfg(test)]
mod tests {
    use super::{Calculadora, escala_visual};

    /// Comprueba los tamaños compacto, mínimo, grande y de proporciones extremas.
    #[test]
    fn escala_se_adapta_a_la_dimension_limitante() {
        for (ancho, alto, esperado) in [
            (360., 520., 1.),
            (280., 400., 400. / 520.),
            (720., 1040., 2.),
            (1920., 1080., 1080. / 520.),
            (360., 2000., 1.),
            (4000., 4000., 3.),
        ] {
            assert!((escala_visual(ancho, alto) - esperado).abs() < 0.001);
        }
    }

    /// Comprueba que la entrada visual se traduce y evalúa con la precedencia correcta.
    #[test]
    fn calcula_expresion_ingresada_con_simbolos_visuales() {
        let mut calculadora = Calculadora::default();
        for token in ["2", "+", "3", "×", "4"] {
            calculadora.insertar(token);
        }

        calculadora.calcular();

        assert_eq!(calculadora.texto_pantalla(), "14");
    }

    /// Comprueba que se puede continuar desde un resultado con otro operador.
    #[test]
    fn permite_encadenar_una_operacion_desde_el_resultado() {
        let mut calculadora = Calculadora::default();
        for token in ["2", "+", "3"] {
            calculadora.insertar(token);
        }
        calculadora.calcular();
        calculadora.insertar("×");
        calculadora.insertar("4");
        calculadora.calcular();

        assert_eq!(calculadora.texto_pantalla(), "20");
    }

    /// Comprueba que los errores se muestran y que una nueva entrada los limpia.
    #[test]
    fn muestra_error_y_se_recupera_con_una_nueva_entrada() {
        let mut calculadora = Calculadora::default();
        for token in ["1", "÷", "0"] {
            calculadora.insertar(token);
        }
        calculadora.calcular();
        assert_eq!(
            calculadora.texto_pantalla(),
            "No se puede dividir entre cero"
        );

        calculadora.insertar("7");
        assert_eq!(calculadora.texto_pantalla(), "7");
    }

    /// Comprueba que borrar restablece la entrada y cualquier mensaje anterior.
    #[test]
    fn borrar_restablece_la_calculadora() {
        let mut calculadora = Calculadora::default();
        calculadora.insertar("9");
        calculadora.calcular();
        calculadora.limpiar();

        assert_eq!(calculadora.texto_pantalla(), "0");
    }
}
