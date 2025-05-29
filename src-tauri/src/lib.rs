mod window;

fn setup(app: &mut tauri::App) {}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            setup(app);
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
