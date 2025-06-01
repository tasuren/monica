import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { platform } from "@tauri-apps/plugin-os";

export interface MouseEventHandler {
    onMouseDown?: () => void;
    onMouseMove?: (x: number, y: number) => void;
    onMouseUp?: () => void;
}

export async function setupMouseEventHandler(
    handler: MouseEventHandler,
): Promise<() => void> {
    const window = await getCurrentWindow();
    const scaleFactor = await window.scaleFactor();

    let unListenMouseDown = () => {};
    let unListenMouseMove = () => {};
    let unListenMouseUp = () => {};

    if (handler.onMouseDown) {
        unListenMouseDown = await listen("mouse-down", (_event) => {
            if (!handler.onMouseDown) return;

            handler.onMouseDown();
        });
    }

    if (handler.onMouseMove) {
        unListenMouseMove = await listen("mouse-move", (event) => {
            if (!handler.onMouseMove) return;

            const [x, y] = event.payload as [number, number];

            if (platform() === "windows") {
                // On windows, the backend (the crate `device_query`) will return
                // the mouse position in physical pixels, so we need to scale it down.
                handler.onMouseMove(x / scaleFactor, y / scaleFactor);
            } else {
                handler.onMouseMove(x, y);
            }
        });
    }

    if (handler.onMouseUp) {
        unListenMouseUp = await listen("mouse-up", () => {
            if (!handler.onMouseUp) return;

            handler.onMouseUp();
        });
    }

    return () => {
        unListenMouseDown();
        unListenMouseMove();
        unListenMouseUp();
    };
}
