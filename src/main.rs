// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::{App, ReadGlobal};

use crate::{
    canvas::{Tool, ToolState},
    canvas_orchestrator::CanvasOrchestrator,
    canvas_window_manager::CanvasWindowManager,
    main_window::MainWindow,
};

mod canvas;
mod canvas_orchestrator;
mod canvas_window;
mod canvas_window_manager;
mod icon;
mod main_window;
mod platform_impl;
mod ui_canvas;
mod ui_main;
mod utils;

const APP_IDENTIFIER: &str = "jp.tasuren.monica";

fn setup(cx: &mut App) {
    gpui_component::init(cx);

    ToolState::register_global(cx, Tool::Cursor, gpui::blue());
    CanvasOrchestrator::register_global(cx);
    CanvasWindowManager::register_global(cx);
    MainWindow::register_global(cx);

    // Quit the application when main window is closed.
    cx.on_window_closed(move |cx| {
        let no_main_window = cx
            .windows()
            .iter()
            .all(|handle| handle.window_id() != MainWindow::global(cx).handle().window_id());

        if no_main_window {
            cx.quit();
        }
    })
    .detach();
}

fn main() {
    gpui::Application::new()
        .with_assets(icon::Assets)
        .run(setup);
}
