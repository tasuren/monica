use gpui::{AnyWindowHandle, App, AppContext, Global, px, size};

use crate::platform_impl::WindowExt;

pub struct MainWindow(AnyWindowHandle);

impl Global for MainWindow {}

impl MainWindow {
    pub fn register_global(cx: &mut App) {
        let window = Self(Self::setup_main_window(cx));
        cx.set_global(window);
    }

    fn setup_main_window(cx: &mut App) -> AnyWindowHandle {
        let titlebar = Some(gpui::TitlebarOptions {
            title: Some("Monica - Controller".into()),
            appears_transparent: true,
            ..Default::default()
        });
        let bounds = gpui::Bounds::centered(None, size(px(230.), px(100.)), cx);
        let window_bounds = Some(gpui::WindowBounds::Windowed(bounds));

        let window_options = gpui::WindowOptions {
            titlebar,
            window_bounds,
            is_resizable: false,
            #[cfg(not(target_os = "windows"))]
            kind: gpui::WindowKind::PopUp,
            #[cfg(target_os = "windows")]
            kind: gpui::WindowKind::Floating, // to allow minimizing the window
            app_id: Some(crate::APP_IDENTIFIER.to_owned()),
            ..Default::default()
        };

        *cx.open_window(window_options, move |window, cx| {
            window.setup_main_window();

            let app_view = crate::ui_main::AppView::new(cx);
            cx.new(|cx| gpui_component::Root::new(app_view, window, cx))
        })
        .expect("Failed to open the main window.")
    }

    pub fn handle(&self) -> AnyWindowHandle {
        self.0
    }

    #[cfg(target_os = "windows")]
    pub fn bring_foreground(&self, cx: &mut App) {
        use crate::platform_impl::windows::WindowsWindowExt;

        self.0
            .update(cx, |_, window, _| window.set_window_pos_top())
            .unwrap();
    }
}
