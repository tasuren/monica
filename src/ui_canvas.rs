use gpui::{App, Entity, MouseMoveEvent, canvas, div, prelude::*};

use crate::canvas::Canvas;

pub struct CanvasView {
    canvas: Entity<Canvas>,
}

impl CanvasView {
    pub fn new(_cx: &mut App, canvas: Entity<Canvas>) -> Self {
        Self { canvas }
    }
}

impl Render for CanvasView {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(gpui::transparent_white())
            .child(
                canvas(|_bounds, _window, _cx| {}, {
                    let canvas = self.canvas.clone();

                    move |_, _, window, cx| {
                        canvas.read(cx).paint(window);
                    }
                })
                .bg(gpui::transparent_white()),
            )
            .on_mouse_move(cx.listener(|view, event: &MouseMoveEvent, _window, cx| {
                if !matches!(event.pressed_button, Some(gpui::MouseButton::Left)) {
                    view.canvas.update(cx, |canvas, cx| {
                        if canvas.is_painting() {
                            canvas.flush(cx);
                        }

                        cx.notify();
                    });

                    return;
                };

                view.canvas.update(cx, |canvas, cx| {
                    canvas.draw(cx, event.position);
                    cx.notify();
                });
            }))
    }
}
