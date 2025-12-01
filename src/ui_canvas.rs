use display_config::DisplayId;
use gpui::{App, Entity, MouseMoveEvent, ReadGlobal, UpdateGlobal, canvas, div, prelude::*};

use crate::{
    canvas::{Tool, ToolState},
    canvas_orchestrator::CanvasOrchestrator,
};

pub struct CanvasView {
    display_id: DisplayId,
}

impl CanvasView {
    pub fn new(cx: &mut App, display_id: DisplayId) -> Entity<Self> {
        CanvasOrchestrator::update_global(cx, {
            let display_id = display_id.clone();

            move |orchestrator, cx| {
                orchestrator.add_canvas(cx, display_id);
            }
        });

        let view = cx.new(|_| Self { display_id });
        cx.observe_release(&view, |view, cx| {
            CanvasOrchestrator::update_global(cx, |orchestrator, _| {
                orchestrator.remove_canvas(&view.display_id);
            });
        })
        .detach();

        view
    }
}

impl Render for CanvasView {
    fn render(&mut self, _window: &mut gpui::Window, cx: &mut Context<Self>) -> impl IntoElement {
        let display_id = self.display_id.clone();

        div()
            .size_full()
            .bg(gpui::transparent_white())
            .child(
                canvas(|_bounds, _window, _cx| {}, {
                    let display_id = display_id.clone();

                    move |_, _, window, cx| {
                        CanvasOrchestrator::update_global(cx, |orchestrator, cx| {
                            orchestrator
                                .update_canvas(cx, &display_id, |canvas, _| canvas.paint(window));
                        });
                    }
                })
                .bg(gpui::transparent_white()),
            )
            .on_mouse_move(cx.listener(move |_, event: &MouseMoveEvent, _, cx| {
                let display_id = display_id.clone();

                CanvasOrchestrator::update_global(cx, move |orchestrator, cx| {
                    // Highlight tool
                    if ToolState::global(cx).tool() == Tool::Highlight {
                        orchestrator.update_canvas(cx, &display_id, |canvas, cx| {
                            canvas.set_highlight(event.position);
                            cx.notify();
                        });
                    }

                    // Canvas draw tool
                    if !matches!(event.pressed_button, Some(gpui::MouseButton::Left)) {
                        orchestrator.update_canvas(cx, &display_id, |canvas, cx| {
                            if canvas.is_painting() {
                                canvas.flush();
                            }

                            cx.notify();
                        });

                        return;
                    };

                    orchestrator.action_canvas(cx, display_id, |canvas, cx| {
                        canvas.draw(cx, event.position);
                        cx.notify();
                    });
                });
            }))
    }
}
