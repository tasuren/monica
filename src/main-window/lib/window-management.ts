import { type Window, getAllWindows } from "@tauri-apps/api/window";
import { setupMouseEventHandler } from "../../common/mouse";
import type { ToolKind } from "../../common/tool";
import {isInsideRect, getWindowPosition, getWindowSize } from "./window-rect-utils";
import { WindowRectTracker, WindowState } from "./window-state";

async function setupWindowDraggingCanvasLock(setLock: (lock: boolean) => void) {
    let dragging = false;
    const onWindowMove = () => {
        // When the window is dragging, we want to lock the draw window.
        if (dragging) {
            dragging = true;
            setLock(true);
        }
    };

    const onMouseDown = () => {
        dragging = true;
    };

    const onMouseUp = () => {
        dragging = false;
    };

    return {
        onWindowMove,
        onMouseDown,
        onMouseUp,
    };
}

async function getCurrentDrawingWindow(x: number, y: number): Promise<Window> {
    for (const window of await getAllWindows()) {
        const position = await getWindowPosition(window);
        const size = await getWindowSize(window);

        if (await isInsideRect(x, y, position, size)) {
            return window;
        }
    }

    throw new Error("No drawing window found.");
}

async function setupWindowFocus(
    state: WindowState,
    opts: { lock: () => boolean; setLock: (lock: boolean) => void },
) {
    const { lock, setLock } = opts;

    const onMouseMove = async (x: number, y: number) => {
        if (state.isDragging()) return;

        if (await state.isInside({ x, y })) {
            if (lock()) return;
            setLock(true);

            state.window.setFocus();
            console.log(state.window.label, "focus");
        } else if (lock()) {
            setLock(false);

            const window = await getCurrentDrawingWindow(x, y);
            window.setFocus();
            console.log(window.label, "focus");
        }
    };

    return {
        onMouseMove,
    };
}

async function setupDrawingWindowFocus(
    tool: () => ToolKind,
    lock: () => boolean,
) {
    let beforeWindow = "";
    const onMouseMove = async (x: number, y: number) => {
        if (lock() || tool() === "cursor") return;

        const window = await getCurrentDrawingWindow(x, y);

        if (beforeWindow !== window.label) {
            console.log(window.label, "focus-switch");
            window.setFocus();
            beforeWindow = window.label;
        }
    };

    return {
        onMouseMove,
    };
}

export async function setupWindowManagement(opts: {
    tool: () => ToolKind;
    lock: () => boolean;
    setLock: (lock: boolean) => void;
}) {
    const rectTracker = new WindowRectTracker();
    await rectTracker.listen();
    const state = new WindowState(rectTracker);

    const canvasLock = await setupWindowDraggingCanvasLock(opts.setLock);
    const windowFocus = await setupWindowFocus(state, opts);
    const drawingWindowFocus = await setupDrawingWindowFocus(
        opts.tool,
        opts.lock,
    );

    const unListenWindowMoved = await state.window.onMoved(async () => {
        canvasLock.onWindowMove();
    });

    const cleanup = await setupMouseEventHandler({
        onMouseDown() {
            canvasLock.onMouseDown();
        },
        onMouseMove(x, y) {
            windowFocus.onMouseMove(x, y);
            drawingWindowFocus.onMouseMove(x, y);
        },
        onMouseUp() {
            canvasLock.onMouseUp();
        },
    });

    return () => {
        cleanup();
        unListenWindowMoved();
        rectTracker.unListen();
    };
}
