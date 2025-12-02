use std::collections::HashMap;

use display_config::{DisplayId, DisplayObserver, Event as DisplayEvent, get_displays};
use gpui::{App, AsyncApp, Global};

use crate::canvas_window::CanvasWindow;

pub struct CanvasWindowManager {
    windows: HashMap<DisplayId, CanvasWindow>,
    _display_observer: DisplayObserver,
}

impl Global for CanvasWindowManager {}

impl CanvasWindowManager {
    pub fn register_global(cx: &mut App) {
        let (tx, rx) = async_channel::unbounded();
        let display_observer = DisplayObserver::new().unwrap();

        display_observer.set_callback(move |event| {
            tx.send_blocking(event).unwrap();
        });

        let manager = Self {
            windows: Self::setup_canvas_windows(cx),
            _display_observer: display_observer,
        };
        cx.set_global(manager);

        cx.spawn(async move |cx| Self::listener(cx, rx).await)
            .detach();

        #[cfg(target_os = "windows")]
        cx.spawn({
            // On windows, `on_mouse_move` event will not be dispatched when the window is not inactive.
            // So we need to manually dispatch the event to all canvases to support the highlight tool.

            async move |cx| Self::dispatch_mouse_move_event_manually(cx).await
        })
        .detach();
    }

    #[cfg(target_os = "windows")]
    async fn dispatch_mouse_move_event_manually(cx: &mut gpui::AsyncApp) {
        use device_query::{DeviceEvents, DeviceEventsHandler};

        let handler = DeviceEventsHandler::new(std::time::Duration::from_millis(10))
            .expect("Failed to create device event handler");

        let (tx, rx) = async_channel::unbounded();
        let _mouse_move_guard = handler.on_mouse_move(move |(x, y)| {
            _ = tx.send_blocking((*x as f32, *y as f32));
        });

        while let Ok((x, y)) = rx.recv().await {
            cx.update_global(|window_manager: &mut Self, cx| {
                for window in window_manager.windows.values() {
                    window.on_mouse_move(cx, x, y);
                }
            })
            .unwrap();
        }
    }

    fn setup_canvas_windows(cx: &mut App) -> HashMap<DisplayId, CanvasWindow> {
        let mut windows = HashMap::new();

        for display in get_displays().unwrap() {
            if display.is_mirrored {
                continue;
            };

            windows.insert(display.id.clone(), CanvasWindow::new(cx, display));
        }

        windows
    }

    async fn listener(cx: &mut AsyncApp, rx: async_channel::Receiver<DisplayEvent>) {
        while let Ok(event) = rx.recv().await {
            cx.update_global(|windows: &mut Self, cx| match event {
                DisplayEvent::Added(display) => {
                    let id = display.id.clone();
                    let window = CanvasWindow::new(cx, display);
                    windows.windows.insert(id, window);
                }
                DisplayEvent::Removed(display_id) => {
                    if let Some(window) = windows.windows.remove(&display_id) {
                        window.close(cx);
                    }
                }
                DisplayEvent::SizeChanged { display, after, .. } => {
                    if let Some(window) = windows.windows.get(&display.id) {
                        window.set_size(cx, after);
                    }
                }
                DisplayEvent::Mirrored(display) => {
                    if let Some(window) = windows.windows.get(&display.id) {
                        window.set_hidden(cx, true);
                    }
                }
                DisplayEvent::UnMirrored(display) => {
                    if let Some(window) = windows.windows.get(&display.id) {
                        window.set_hidden(cx, false);
                    }
                }
                _ => {}
            })
            .unwrap();
        }
    }

    pub fn set_action_mode(&self, cx: &mut App, action_mode: bool) {
        for window in self.windows.values() {
            window.set_ignore_cursor_events(cx, action_mode);
        }
    }
}
