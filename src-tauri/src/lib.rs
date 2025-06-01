use tauri::Manager;

mod mouse;
mod window;

fn setup(app: &mut tauri::App) {
    window::setup_windows(app);
    mouse::setup_mouse_event_listener(app.app_handle().clone()).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![mouse::get_mouse_position])
        .setup(|app| {
            setup(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
