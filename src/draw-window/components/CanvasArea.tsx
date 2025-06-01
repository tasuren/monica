import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { createEffect, onCleanup, onMount } from "solid-js";
import { useCanvas, useLock, useTool } from "../CanvasController";
import { Canvas } from "../lib/canvas";

export function CanvasArea() {
    // Canvas state management
    let canvasElement!: HTMLCanvasElement;
    const [_, setCanvas] = useCanvas();

    onMount(() => {
        canvasElement.width = window.innerWidth;
        canvasElement.height = window.innerHeight;

        setCanvas(new Canvas(canvasElement));
    });

    // Window cursor events
    createEffect(() => {
        const window = getCurrentWindow();

        if (tool().kind === "cursor" || lock()) {
            window.setIgnoreCursorEvents(true);
            canvasElement.classList.remove("cursor-crosshair");
        } else {
            canvasElement.classList.add("cursor-crosshair");
            window.setIgnoreCursorEvents(false);
            window.setFocus();
        }
    });

    // Tool control
    const [tool] = useTool();
    const [lock] = useLock();

    createEffect(async () => {
        const unListenMouseDown = await listen("mouse-down", (event) => {
            if (!lock()) {
                const [x, y, _] = event.payload as [number, number, boolean];
                tool().down();
                tool().move(x, y);
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
                tool().move(x, y);
            }
        });

        const unListenMouseUp = await listen("mouse-up", () => {
            if (!lock()) {
                tool().up();
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
