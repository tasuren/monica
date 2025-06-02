use std::sync::{atomic::Ordering, Arc};

use anyhow::{Context as _, Result};
use device_query::{DeviceEvents, DeviceEventsHandler, DeviceQuery, DeviceState};
use tauri::Emitter;

pub fn setup_mouse_event_listener(app: tauri::AppHandle) -> Result<()> {
    let device_events_handler = DeviceEventsHandler::new(std::time::Duration::from_millis(10))
        .context("Mouse event handler already initialized")?;

    let is_mouse_down = Arc::new(std::sync::atomic::AtomicBool::new(false));

    let on_mouse_down = device_events_handler.on_mouse_down({
        let is_mouse_down = Arc::clone(&is_mouse_down);
        let app = app.clone();

        move |_| {
            is_mouse_down.store(true, Ordering::SeqCst);

            let device_state = DeviceState::new();
            app.emit("mouse-down", device_state.get_mouse().coords)
                .expect("Failed to emit mouse down event");
        }
    });

    let on_mouse_up = device_events_handler.on_mouse_up({
        let is_mouse_down = Arc::clone(&is_mouse_down);
        let app = app.clone();

        move |_| {
            is_mouse_down.store(false, Ordering::SeqCst);

            let device_state = DeviceState::new();
            app.emit("mouse-up", device_state.get_mouse().coords)
                .expect("Failed to emit mouse up event");
        }
    });

    let on_mouse_move = device_events_handler.on_mouse_move({
        let is_mouse_down = Arc::clone(&is_mouse_down);

        move |(x, y)| {
            app.emit("mouse-move", (x, y, is_mouse_down.load(Ordering::SeqCst)))
                .expect("Failed to emit mouse move event");
        }
    });

    // Forget the callbacks to prevent them from being dropped
    // and to keep the event handler running during the application lifetime.
    std::mem::forget((on_mouse_down, on_mouse_up, on_mouse_move));

    Ok(())
}

#[tauri::command]
pub fn get_mouse_position() -> (i32, i32) {
    let device_state = DeviceState::new();
    device_state.get_mouse().coords
}
