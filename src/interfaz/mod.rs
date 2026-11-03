//! Presentación de la calculadora mediante una interfaz nativa de GPUI.

use gpui::{
    App, Bounds, Context, TitlebarOptions, Window, WindowBounds, WindowOptions, div, prelude::*,
    px, rgb, size,
};
use gpui_platform::application;

/// Vista estática de la calculadora básica, sin estado ni acciones asociadas.
pub struct Calculadora;

impl Render for Calculadora {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let escala = escala_visual(
            f32::from(window.viewport_size().width),
            f32::from(window.viewport_size().height),
        );
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
                    .grid_rows(6)
                    .gap(px(8. * escala))
                    .child(
                        div()
                            .col_span_full()
                            .w_full()
                            .h_full()
                            .min_h_0()
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_lg()
                            .bg(rgb(0x111214))
                            .text_size(px(32. * escala))
                            .text_color(rgb(0xffffff))
                            .child("0"),
                    )
                    .child(tecla("C", 1, escala))
                    .child(tecla("√", 1, escala))
                    .child(tecla("%", 1, escala))
                    .child(tecla("÷", 1, escala))
                    .child(tecla("7", 1, escala))
                    .child(tecla("8", 1, escala))
                    .child(tecla("9", 1, escala))
                    .child(tecla("×", 1, escala))
                    .child(tecla("4", 1, escala))
                    .child(tecla("5", 1, escala))
                    .child(tecla("6", 1, escala))
                    .child(tecla("−", 1, escala))
                    .child(tecla("1", 1, escala))
                    .child(tecla("2", 1, escala))
                    .child(tecla("3", 1, escala))
                    .child(tecla("+", 1, escala))
                    .child(tecla("0", 2, escala))
                    .child(tecla(".", 1, escala))
                    .child(tecla("=", 1, escala)),
            )
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
            |_, cx| cx.new(|_| Calculadora),
        );
        if let Err(error) = resultado {
            eprintln!("No se pudo abrir la ventana de la calculadora: {error}");
            cx.quit();
            return;
        }
        cx.activate(true);
    });
}

/// Construye una tecla visual que ocupa su celda sin ejecutar operaciones.
fn tecla(etiqueta: &'static str, columnas: u16, escala: f32) -> impl IntoElement {
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
    use super::escala_visual;

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
}
