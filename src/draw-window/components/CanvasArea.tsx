import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { createEffect, onCleanup, onMount } from "solid-js";
import { useCanvas, useLock, useTool } from "../GlobalState";
import { Canvas } from "../lib/canvas";
import { ToolController } from "../lib/tool-controller";

export function CanvasArea() {
    // Canvas state management
    let canvasElement!: HTMLCanvasElement;
    const [canvas, setCanvas] = useCanvas();

    onMount(() => {
        canvasElement.width = window.innerWidth;
        canvasElement.height = window.innerHeight;

        setCanvas(new Canvas(canvasElement));
    });

    // Window cursor events
    createEffect(() => {
        const window = getCurrentWindow();

        if (tool() === "cursor") {
            window.setIgnoreCursorEvents(true);
            canvasElement.classList.remove("cursor-crosshair");
        } else {
            window.setIgnoreCursorEvents(false);
            canvasElement.classList.add("cursor-crosshair");
        }
    });

    // Tool controller
    const [tool] = useTool();
    const [lock] = useLock();

    createEffect(async () => {
        const tools = new ToolController(canvas(), tool);

        const unListenMouseDown = await listen("mouse-down", (event) => {
            if (!lock()) {
                const [x, y, _] = event.payload as [number, number, boolean];
                tools.onMouseDown(x, y);
            }
        });

        const unListenMouseMove = await listen("mouse-move", (event) => {
            if (!lock()) {
                const [x, y, isMouseDown] = event.payload as [
                    number,
                    number,
                    boolean,
                ];

                if (!isMouseDown) return;
                tools.onMouseMoveWithDown(x, y);
            }
        });

        const unListenMouseUp = await listen("mouse-up", () => {
            if (!lock()) {
                tools.onMouseUp();
            }
        });

        onCleanup(() => {
            unListenMouseDown();
            unListenMouseMove();
            unListenMouseUp();
        });
    });

    return (
        <canvas
            class="w-screen h-screen cursor-crosshair"
            ref={canvasElement}
        />
    );
}
