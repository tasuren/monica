use std::{
    mem::forget,
    sync::{atomic::Ordering, Arc},
};

use anyhow::{Context as _, Result};
use device_query::{device_state, DeviceEvents, DeviceEventsHandler, DeviceQuery, DeviceState};
use tauri::Emitter;

pub fn setup_mouse_event_listener(app: tauri::AppHandle) -> Result<()> {
    let device_events_handler = DeviceEventsHandler::new(std::time::Duration::from_millis(10))
        .context("Mouse event handler already initialized")?;

    let is_mouse_down = Arc::new(std::sync::atomic::AtomicBool::new(false));

    let on_mouse_down = device_events_handler.on_mouse_down({
        let is_mouse_down = Arc::clone(&is_mouse_down);
        let device_state = DeviceState::new();
        let app = app.clone();

        move |_| {
            is_mouse_down.store(true, Ordering::SeqCst);

            app.emit("mouse-down", device_state.get_mouse().coords)
                .expect("Failed to emit mouse down event");
        }
    });

    let on_mouse_up = device_events_handler.on_mouse_up({
        let is_mouse_down = Arc::clone(&is_mouse_down);
        let device_state = DeviceState::new();
        let app = app.clone();

        move |_| {
            is_mouse_down.store(false, Ordering::SeqCst);

            app.emit("mouse-up", device_state.get_mouse().coords)
                .expect("Failed to emit mouse up event");
        }
    });

    let on_mouse_move = device_events_handler.on_mouse_move({
        let is_mouse_down = Arc::clone(&is_mouse_down);

        move |(x, y)| {
            if !is_mouse_down.load(Ordering::SeqCst) {
                return;
            }

            app.emit("mouse-move", (x, y))
                .expect("Failed to emit mouse move event");
        }
    });

    // Prevent the callback guards from being dropped.
    // These callback guards will be used during the lifetime of the application.
    forget(on_mouse_down);
    forget(on_mouse_up);
    forget(on_mouse_move);

    Ok(())
}
