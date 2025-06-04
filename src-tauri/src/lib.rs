use tauri::Manager;

mod mouse;
mod permissions;
mod window;

fn setup(app: &mut tauri::App) {
    tauri::async_runtime::block_on(permissions::check_permissions(app));

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    window::setup_windows(app);
    mouse::setup_mouse_event_listener(app.app_handle().clone()).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_macos_permissions::init())
        .invoke_handler(tauri::generate_handler![mouse::get_mouse_position])
        .setup(|app| {
            setup(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
