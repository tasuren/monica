use gpui::{div, prelude::*};

pub struct AppView {}

impl Render for AppView {
    fn render(&mut self, window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child("hello")
    }
}
