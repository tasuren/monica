use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

pub async fn check_permissions(app: &tauri::App) {
    #[cfg(target_os = "macos")]
    {
        if !tauri_plugin_macos_permissions::check_accessibility_permission().await {
            tauri_plugin_macos_permissions::request_accessibility_permission().await;

            app.dialog()
                .message(
                    "Please enable the accessibility permission for this app.\n\
                    And please restart the app."
                )
                .buttons(MessageDialogButtons::Ok)
                .blocking_show();

            std::process::exit(0);
        }
    }
}
