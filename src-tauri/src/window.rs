#[cfg(target_os = "macos")]
fn setup_macos_window(window: &tauri::WebviewWindow) {
    use objc2::rc::Retained;
    use objc2_app_kit::NSWindow;

    let ptr = window.ns_window().expect("Failed to get NSWindow");
    // SAFETY: We are assuming that the pointer is valid and correctly typed.
    // It is derived from a Tauri `WebviewWindow`, which is expected to be an `NSWindow`.
    let ns_window = unsafe { Retained::from_raw(ptr as *mut NSWindow).unwrap() };

    // Set the window level to status bar level to ensure it appears above the menu bar.
    ns_window.setLevel(objc2_app_kit::NSStatusWindowLevel);
}

pub fn setup_windows(app: &mut tauri::App) {
    let monitors = app
        .available_monitors()
        .expect("Failed to get available monitors.");

    #[cfg(debug_assertions)]
    let primary_monitor = app
        .primary_monitor()
        .expect("Failed to get primary monitor.")
        .unwrap();

    for (i, monitor) in monitors.into_iter().enumerate() {
        let tauri::LogicalSize { width, height } =
            monitor.size().to_logical::<f64>(monitor.scale_factor());
        let tauri::LogicalPosition { x, y } =
            monitor.position().to_logical::<f64>(monitor.scale_factor());

        let window = tauri::WebviewWindowBuilder::new(
            app,
            format!("draw-{i}"),
            tauri::WebviewUrl::App("/".into()),
        )
        .title(format!("Monica Draw Window ({i})"))
        .hidden_title(true)
        .decorations(false)
        .transparent(true)
        .inner_size(width, height)
        .position(x, y)
        .always_on_top(true)
        .resizable(false)
        .build()
        .expect("Failed to create window");

        if !DISABLE_IGNORE_CURSOR_EVENTS {
            window.set_ignore_cursor_events(true).unwrap();
        }

        #[cfg(target_os = "macos")]
        setup_macos_window(&window);

        // For debugging
        if OPEN_ALL_DRAWING_DEVTOOLS
            || cfg!(debug_assertions) && monitor.name() == primary_monitor.name()
        {
            window.open_devtools();
        }
    }
}

const OPEN_ALL_DRAWING_DEVTOOLS: bool = option_env!("OPEN_ALL_DRAWING_DEVTOOLS").is_some();
const DISABLE_IGNORE_CURSOR_EVENTS: bool = option_env!("DISABLE_IGNORE_CURSOR_EVENTS").is_some();
