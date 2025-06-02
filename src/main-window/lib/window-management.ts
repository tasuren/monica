import {
    type LogicalPosition,
    type LogicalSize,
    type Window,
    getAllWindows,
} from "@tauri-apps/api/window";
import { setupMouseEventHandler } from "../../common/mouse";
import type { ToolKind } from "../../common/tool";
import {
    getWindowPosition,
    getWindowSize,
    isInsideRect,
} from "./window-rect-utils";
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

let cache:
    | {
          window: Window;
          position: LogicalPosition;
          size: LogicalSize;
      }[]
    | undefined = undefined;
async function getDrawingWindowRects() {
    if (cache === undefined) {
        cache = [];

        for (const window of await getAllWindows()) {
            const position = await getWindowPosition(window);
            const size = await getWindowSize(window);

            cache.push({ window, position, size });
        }
    }

    return cache;
}

async function getCurrentDrawingWindow(x: number, y: number): Promise<Window> {
    for (const rect of await getDrawingWindowRects()) {
        if (await isInsideRect(x, y, rect.position, rect.size)) {
            return rect.window;
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
        console.log(1);

        if (await state.isInside({ x, y })) {
            if (lock()) return;
            setLock(true);

            state.window.setFocus();
        } else if (lock()) {
            setLock(false);

            const window = await getCurrentDrawingWindow(x, y);
            window.setFocus();
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
