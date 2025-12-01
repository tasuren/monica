use display_config::{Display, DisplayId};
use gpui::{AnyWindowHandle, App};

use crate::{
    platform_impl::WindowExt,
    ui_canvas::CanvasView,
    utils::{self, dpi_size_to_gpui},
};

pub struct CanvasWindow {
    _display_id: DisplayId,
    window_handle: AnyWindowHandle,
}

#[inline]
fn setup_canvas_window(cx: &mut App, display: Display) -> AnyWindowHandle {
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

    *cx.open_window(window_options, move |window, cx| {
        window.set_ignore_cursor_events(true);

        #[cfg(target_os = "macos")]
        {
            use crate::platform_impl::macos::MacOSWindowExt;

            window.setup_canvas_window();
        }

        CanvasView::new(cx, display.id)
    })
    .expect("Failed to open paint window")
}

impl CanvasWindow {
    pub fn new(cx: &mut App, display: Display) -> Self {
        Self {
            _display_id: display.id.clone(),
            window_handle: setup_canvas_window(cx, display),
        }
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
