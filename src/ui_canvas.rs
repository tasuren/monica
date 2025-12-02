use display_config::DisplayId;
use gpui::{
    App, Entity, MouseMoveEvent, Pixels, Point, ReadGlobal, UpdateGlobal, canvas, div, prelude::*,
};

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

        #[cfg(target_os = "windows")]
        cx.spawn({
            // On windows, `on_mouse_move` event will not be dispatched when the window is not inactive.
            // So we need to dispatch the event manually to support the highlight tool.

            let view = view.clone();

            async move |cx| {
                use device_query::{DeviceEvents, DeviceEventsHandler};
                use gpui::{point, px};

                let handler = DeviceEventsHandler::new(std::time::Duration::from_millis(10))
                    .expect("Failed to create device event handler.");

                let (tx, rx) = async_channel::unbounded();
                let _mouse_move_guard = handler.on_mouse_move(move |(x, y)| {
                    _ = tx.send_blocking(point(px(*x as _), px(*y as _)));
                });

                while let Ok(mouse_pos) = rx.recv().await {
                    cx.update_entity(&view, |view, cx| {
                        CanvasOrchestrator::update_global(cx, |orchestrator, cx| {
                            view.on_mouse_move_whenever_window_inactive(
                                cx,
                                orchestrator,
                                mouse_pos,
                            );
                        });
                    })
                    .unwrap();
                }
            }
        })
        .detach();

        cx.observe_release(&view, |view, cx| {
            CanvasOrchestrator::update_global(cx, |orchestrator, _| {
                orchestrator.remove_canvas(&view.display_id);
            });
        })
        .detach();

        view
    }

    pub fn on_mouse_move_whenever_window_inactive(
        &self,
        cx: &mut App,
        orchestrator: &mut CanvasOrchestrator,
        mouse_pos: Point<Pixels>,
    ) {
        if ToolState::global(cx).tool() == Tool::Highlight {
            orchestrator.update_canvas(cx, &self.display_id, |canvas, cx| {
                canvas.set_highlight(mouse_pos);
                cx.notify();
            });
        }
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
            .on_mouse_move(cx.listener(move |_view, event: &MouseMoveEvent, _, cx| {
                let display_id = display_id.clone();

                CanvasOrchestrator::update_global(cx, move |orchestrator, cx| {
                    orchestrator.notify_old_working_canvas(cx, Some(&display_id));

                    #[cfg(not(target_os = "windows"))]
                    _view.on_mouse_move_whenever_window_inactive(cx, orchestrator, event.position);

                    // Canvas draw tool
                    if matches!(event.pressed_button, Some(gpui::MouseButton::Left)) {
                        orchestrator.update_canvas(cx, &display_id, |canvas, cx| {
                            canvas.draw(cx, event.position);
                            cx.notify();
                        });
                    } else {
                        orchestrator.action_canvas(cx, display_id, |canvas, cx| {
                            let result = if canvas.is_painting() {
                                canvas.flush();

                                // On windows, the canvas window comes to the front over the main window.
                                // This prevents interaction with the main window,
                                // so we implement processing to bring the main window back to the front.
                                #[cfg(target_os = "windows")]
                                {
                                    use crate::main_window::MainWindow;

                                    MainWindow::update_global(cx, |window, cx| {
                                        window.bring_foreground(cx)
                                    });
                                }

                                true
                            } else {
                                false
                            };

                            cx.notify();

                            result
                        });
                    };
                });
            }))
    }
}
