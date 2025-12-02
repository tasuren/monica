use display_config::{Display, DisplayId};
use gpui::{AnyWindowHandle, App, Entity};

use crate::{
    platform_impl::WindowExt,
    ui_canvas::CanvasView,
    utils::{self, dpi_size_to_gpui},
};

pub struct CanvasWindow {
    _display_id: DisplayId,
    window_handle: AnyWindowHandle,
    _view: Entity<CanvasView>,
}

impl CanvasWindow {
    pub fn new(cx: &mut App, display: Display) -> Self {
        let display_id = display.id.clone();
        let (window_handle, view) = Self::setup_canvas_window(cx, display);

        Self {
            _display_id: display_id,
            window_handle,
            _view: view,
        }
    }

    fn setup_canvas_window(
        cx: &mut App,
        display: Display,
    ) -> (AnyWindowHandle, Entity<CanvasView>) {
        let bounds = gpui::Bounds::new(
            utils::dpi_pos_to_gpui(display.origin),
            utils::dpi_size_to_gpui(display.size),
        );
        let window_bounds = Some(gpui::WindowBounds::Windowed(bounds));

        let window_options = gpui::WindowOptions {
            titlebar: None,
            kind: gpui::WindowKind::PopUp,
            app_id: Some(crate::APP_IDENTIFIER.to_owned()),
            window_background: gpui::WindowBackgroundAppearance::Transparent,
            window_bounds,
            focus: false,
            ..Default::default()
        };

        let mut created_view = std::cell::OnceCell::new();
        let handle = *cx
            .open_window(window_options, |window, cx| {
                window.setup_canvas_window();

                #[cfg(target_os = "windows")]
                {
                    // NOTE: `window_bounds` is not working on Windows so we move the window manually.

                    use crate::platform_impl::windows::WindowsWindowExt;

                    let origin = display.origin;
                    let size = display.size;
                    window.set_window_rect(origin.x, origin.y, size.width as _, size.height as _);
                }

                let view = CanvasView::new(cx, window.window_handle(), display.id);
                created_view.set(view.clone()).unwrap();

                view
            })
            .expect("Failed to open paint window");

        (handle, created_view.take().unwrap())
    }

    #[cfg(target_os = "windows")]
    pub fn on_mouse_move(&self, cx: &mut App, x: f32, y: f32) {
        use gpui::{AppContext, UpdateGlobal, point, px};

        use crate::canvas_orchestrator::CanvasOrchestrator;

        let (window_bounds, scale_factor) = cx
            .update_window(self.window_handle, |_, window, _| {
                (window.bounds(), window.scale_factor())
            })
            .unwrap();

        let mut mouse_pos = point(px(x / scale_factor), px(y / scale_factor));
        if !window_bounds.contains(&mouse_pos) {
            // If there are no mouse on this canvas window, do nothing.
            return;
        }

        mouse_pos.x -= window_bounds.origin.x;
        mouse_pos.y -= window_bounds.origin.y;

        cx.update_entity(&self._view, |view, cx| {
            CanvasOrchestrator::update_global(cx, |orchestrator, cx| {
                view.on_mouse_move_whenever_window_inactive(cx, orchestrator, mouse_pos);
            });
        });
    }

    pub fn set_ignore_cursor_events(&self, cx: &mut App, ignore: bool) {
        self.window_handle
            .update(cx, move |_, window, _| {
                window.set_ignore_cursor_events(ignore)
            })
            .unwrap();
    }

    pub fn set_size(&self, cx: &mut App, size: dpi::LogicalSize<u32>) {
        let size = dpi_size_to_gpui(size);

        self.window_handle
            .update(cx, move |_, window, _| {
                window.resize(size);
                // window.refresh();
            })
            .unwrap();
    }

    pub fn set_hidden(&self, cx: &mut App, hidden: bool) {
        self.window_handle
            .update(cx, move |_, window, _| window.set_hidden(hidden))
            .unwrap();
    }

    pub fn close(&self, cx: &mut App) {
        self.window_handle
            .update(cx, |_, window, _| window.remove_window())
            .unwrap();
    }
}
