// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::{App, AppContext, WindowId, px, size};

use crate::{
    canvas::{Tool, ToolState},
    canvas_orchestrator::CanvasOrchestrator,
    canvas_window_manager::CanvasWindowManager,
};

mod canvas;
mod canvas_orchestrator;
mod canvas_window;
mod canvas_window_manager;
mod icon;
mod platform_impl;
mod ui_canvas;
mod ui_main;
mod utils;

const APP_IDENTIFIER: &str = "jp.tasuren.monica";

fn setup(cx: &mut App) {
    gpui_component::init(cx);

    CanvasOrchestrator::register_global(cx);
    CanvasWindowManager::register_global(cx);
    ToolState::register_global(cx, Tool::Cursor, gpui::blue());

    let main_window_id = setup_main_window(cx);

    // Quit the application when main window is closed.
    cx.on_window_closed(move |cx| {
        let no_main_window = cx
            .windows()
            .iter()
            .all(|handle| handle.window_id() != main_window_id);

        if no_main_window {
            cx.quit();
        }
    })
    .detach();
}

fn setup_main_window(cx: &mut App) -> WindowId {
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
        kind: gpui::WindowKind::PopUp,
        app_id: Some(APP_IDENTIFIER.to_owned()),
        ..Default::default()
    };

    let main_window_handle = cx
        .open_window(window_options, move |window, cx| {
            #[cfg(target_os = "macos")]
            {
                use platform_impl::macos::MacOSWindowExt;

                window.setup_main_window();
            }

            let app_view = ui_main::AppView::new(cx);
            cx.new(|cx| gpui_component::Root::new(app_view, window, cx))
        })
        .expect("Failed to open the main window.");

    main_window_handle.window_id()
}

fn main() {
    gpui::Application::new()
        .with_assets(icon::Assets)
        .run(setup);
}
