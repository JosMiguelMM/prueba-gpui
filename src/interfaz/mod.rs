//! Presentación de la calculadora mediante una interfaz nativa de GPUI.

use gpui::{
    App, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use gpui_platform::application;

/// Vista estática de la calculadora básica, sin estado ni acciones asociadas.
pub struct Calculadora;

impl Render for Calculadora {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_3()
            .p_4()
            .bg(rgb(0x202124))
            .child(
                div()
                    .w_full()
                    .h(px(112.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_lg()
                    .bg(rgb(0x111214))
                    .text_3xl()
                    .text_color(rgb(0xffffff))
                    .child("0"),
            )
            .child(fila_teclas(["7", "8", "9", "÷"]))
            .child(fila_teclas(["4", "5", "6", "×"]))
            .child(fila_teclas(["1", "2", "3", "−"]))
            .child(fila_teclas(["C", "0", ".", "+"]))
            .child(tecla("=", true))
    }
}

/// Inicia GPUI y abre una ventana centrada con la vista de la calculadora.
pub fn iniciar() {
    application().run(|cx: &mut App| {
        let bounds = WindowBounds::Windowed(Bounds::centered(None, size(px(360.), px(520.)), cx));
        cx.open_window(
            WindowOptions {
                window_bounds: Some(bounds),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Calculadora),
        )
        .expect("No se pudo abrir la ventana de la calculadora");
        cx.activate(true);
    });
}

fn fila_teclas(etiquetas: [&'static str; 4]) -> impl IntoElement {
    div()
        .w_full()
        .flex()
        .flex_row()
        .gap_2()
        .children(etiquetas.into_iter().map(|etiqueta| tecla(etiqueta, false)))
}

fn tecla(etiqueta: &'static str, ancho_completo: bool) -> impl IntoElement {
    div()
        .when(ancho_completo, |elemento| elemento.w_full())
        .when(!ancho_completo, |elemento| elemento.flex_1())
        .h(px(56.))
        .flex()
        .items_center()
        .justify_center()
        .rounded_md()
        .border_1()
        .border_color(rgb(0x45474a))
        .bg(rgb(0x303236))
        .text_2xl()
        .text_color(rgb(0xffffff))
        .child(etiqueta)
}
