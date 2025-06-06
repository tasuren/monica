use tauri::Manager;

mod mouse;
mod permissions;
mod window;

fn setup(app: &mut tauri::App) {
    #[cfg(target_os = "macos")]
    tauri::async_runtime::block_on(permissions::macos::check_permissions(app));

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    window::setup_windows(app);
    mouse::setup_mouse_event_listener(app.app_handle().clone()).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(target_os = "macos")]
    {
        builder = builder
            .plugin(tauri_plugin_dialog::init())
            .plugin(tauri_plugin_macos_permissions::init());
    }

    builder
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
