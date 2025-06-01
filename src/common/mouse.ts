import { listen } from "@tauri-apps/api/event";

export interface MouseEventHandler {
    onMouseDown?: () => void;
    onMouseMove?: (x: number, y: number) => void;
    onMouseUp?: () => void;
}

export async function setupMouseEventHandler(
    handler: MouseEventHandler,
): Promise<() => void> {
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
            handler.onMouseMove(x, y);
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
