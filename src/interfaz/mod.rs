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
            .grid()
            .grid_cols(4)
            .gap_2()
            .p_4()
            .bg(rgb(0x202124))
            .child(
                div()
                    .col_span_full()
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
            .child(tecla("C", 1))
            .child(tecla("√", 1))
            .child(tecla("%", 1))
            .child(tecla("÷", 1))
            .child(tecla("7", 1))
            .child(tecla("8", 1))
            .child(tecla("9", 1))
            .child(tecla("×", 1))
            .child(tecla("4", 1))
            .child(tecla("5", 1))
            .child(tecla("6", 1))
            .child(tecla("−", 1))
            .child(tecla("1", 1))
            .child(tecla("2", 1))
            .child(tecla("3", 1))
            .child(tecla("+", 1))
            .child(tecla("0", 2))
            .child(tecla(".", 1))
            .child(tecla("=", 1))
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

fn tecla(etiqueta: &'static str, columnas: u16) -> impl IntoElement {
    div()
        .when(columnas > 1, |elemento| elemento.col_span(columnas))
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
