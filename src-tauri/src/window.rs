use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

#[cfg(target_os = "macos")]
mod macos {
    use objc2::rc::Retained;
    use objc2_app_kit::NSWindow;

    pub fn get_ns_window(window: &tauri::WebviewWindow) -> Retained<NSWindow> {
        let ptr = window.ns_window().expect("Failed to get NSWindow");
        // SAFETY: We are assuming that the pointer is valid and correctly typed.
        // It is derived from a Tauri `WebviewWindow`, which is expected to be an `NSWindow`.
        unsafe { Retained::from_raw(ptr as *mut NSWindow).unwrap() }
    }

    #[cfg(target_os = "macos")]
    pub fn setup_macos_drawing_window(window: &tauri::WebviewWindow) {
        let ns_window = get_ns_window(window);

        // Set the window level to status bar level to ensure it appears above the menu bar.
        ns_window.setLevel(objc2_app_kit::NSStatusWindowLevel);
    }

    #[cfg(target_os = "macos")]
    pub fn setup_macos_main_window(window: &tauri::WebviewWindow) {
        let ns_window = get_ns_window(window);

        ns_window.setLevel(objc2_app_kit::NSStatusWindowLevel + 1);
    }
}

pub fn setup_windows(app: &mut tauri::App) {
    // Main window
    let window = app.get_webview_window("main").unwrap();
    window
        .set_position(tauri::LogicalPosition::new(60., 80.))
        .expect("Failed to set position");

    #[cfg(target_os = "macos")]
    macos::setup_macos_main_window(&window);

    // Drawing windows
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

        let window =
            WebviewWindowBuilder::new(app, format!("draw-{i}"), WebviewUrl::App("/".into()))
                .title(format!("Monica Draw Window ({i})"))
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
        macos::setup_macos_drawing_window(&window);

        // For debugging
        #[cfg(debug_assertions)]
        if OPEN_ALL_DRAWING_DEVTOOLS || monitor.position() == primary_monitor.position() {
            window.open_devtools();
        }
    }
}

#[cfg(debug_assertions)]
const OPEN_ALL_DRAWING_DEVTOOLS: bool = option_env!("OPEN_ALL_DRAWING_DEVTOOLS").is_some();
const DISABLE_IGNORE_CURSOR_EVENTS: bool = option_env!("DISABLE_IGNORE_CURSOR_EVENTS").is_some();
