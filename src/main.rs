use display_config::get_displays;
use gpui::{App, AppContext, UpdateGlobal, WindowId, point, px, size};

use crate::{
    canvas::{GlobalState, Tool},
    platform_impl::WindowExt,
};

mod canvas;
mod icon;
mod platform_impl;
mod ui_canvas;
mod ui_main;

const APP_IDENTIFIER: &str = "jp.tasuren.monica";

fn setup(cx: &mut App) {
    gpui_component::init(cx);
    GlobalState::set_global(cx, GlobalState::new(Tool::Cursor, gpui::blue()));

    let main_window_id = setup_windows(cx);

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

#[inline]
fn setup_windows(cx: &mut App) -> WindowId {
    // canvas window
    for display in get_displays().expect("Failed to get all displays.") {
        let bounds = gpui::Bounds::new(
            point(px(display.origin.x as _), px(display.origin.y as _)),
            size(px(display.size.width as _), px(display.size.height as _)),
        );
        let window_bounds = Some(gpui::WindowBounds::Windowed(bounds));

        let window_options = gpui::WindowOptions {
            titlebar: None,
            kind: gpui::WindowKind::PopUp,
            app_id: Some(APP_IDENTIFIER.to_owned()),
            window_background: gpui::WindowBackgroundAppearance::Transparent,
            window_bounds,
            focus: false,
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            let canvas = GlobalState::update_global(cx, |state, cx| {
                state.canvas_manager().create_canvas(cx, window, display.id)
            });

            window.set_most_top();
            window.set_ignore_cursor_events(true);

            cx.new(|cx| ui_canvas::CanvasView::new(cx, canvas))
        })
        .expect("Failed to open paint window");
    }

    // main window
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
        .open_window(window_options, |window, cx| {
            let app_view = cx.new(ui_main::AppView::new);
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
