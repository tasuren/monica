use gpui::prelude::*;
use gpui_component::{Icon, h_flex};

pub struct AppView {}

impl AppView {
    pub fn new(_cx: &mut Context<'_, Self>) -> Self {
        Self {}
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .size_full()
            .justify_around()
            .items_center()
            .gap_2()
            .p_1()
            .child(Icon::empty().path("icons/grip.svg").size_7())
            .child(Icon::empty().path("icons/mouse-pointer-2.svg").size_7())
            .child(Icon::empty().path("icons/pencil.svg").size_7())
            .child(Icon::empty().path("icons/eraser.svg").size_7())
            .child(Icon::empty().path("icons/circle.svg").size_7())
            .child(Icon::empty().path("icons/trash-2.svg").size_7())
            .child(Icon::empty().path("icons/x.svg").size_7())
    }
}
