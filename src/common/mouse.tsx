import { listen } from "@tauri-apps/api/event";

export interface MouseEventHandler {
    onMouseDown?: () => void;
    onMouseMove?: (x: number, y: number) => void;
    onMouseUp?: () => void;
}

export async function setupMouseEventHandler(
    handler: MouseEventHandler,
): Promise<() => void> {
    const unListenMouseDown = await listen("mouse-down", (_event) => {
        if (handler.onMouseDown) {
            handler.onMouseDown();
        }
    });
    const unListenMouseMove = await listen("mouse-move", (event) => {
        if (!handler.onMouseMove) return;

        const [x, y] = event.payload as [number, number];
        handler.onMouseMove(x, y);
    });
    const unListenMouseUp = await listen("mouse-up", () => {
        if (handler.onMouseUp) {
            handler.onMouseUp();
        }
    });

    return () => {
        unListenMouseDown();
        unListenMouseMove();
        unListenMouseUp();
    };
}
