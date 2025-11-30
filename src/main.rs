use gpui::{App, AppContext, px, size};

mod canvas;
mod icon;
mod ui;

const APP_IDENTIFIER: &str = "jp.tasuren.monica";

fn setup(cx: &mut App) {
    gpui_component::init(cx);

    setup_window(cx);
}

#[inline]
fn setup_window(cx: &mut App) {
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

    cx.open_window(window_options, |window, cx| {
        let app_view = cx.new(ui::AppView::new);
        cx.new(|cx| gpui_component::Root::new(app_view, window, cx))
    })
    .expect("Failed to open the main window.");
}

fn main() {
    gpui::Application::new()
        .with_assets(icon::Assets)
        .run(setup);
}
