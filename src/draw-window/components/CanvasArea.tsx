import { getCurrentWindow } from "@tauri-apps/api/window";
import { createEffect, createMemo, onCleanup, onMount } from "solid-js";
import { useCanvas, useLock, useTool } from "../CanvasController";
import { Canvas } from "../lib/canvas";

export function CanvasArea() {
    const [tool] = useTool();
    const [lock] = useLock();

    // Canvas state management
    let canvasElement!: HTMLCanvasElement;
    const [_, setCanvas] = useCanvas();

    onMount(() => {
        canvasElement.width = window.innerWidth;
        canvasElement.height = window.innerHeight;

        setCanvas(new Canvas(canvasElement));
    });

    // Window cursor events
    const drawing = createMemo(() => tool().kind === "cursor" || lock());

    createEffect(() => {
        const window = getCurrentWindow();

        if (drawing()) {
            window.setIgnoreCursorEvents(true);
            canvasElement.classList.remove("cursor-crosshair");
        } else {
            canvasElement.classList.add("cursor-crosshair");
            window.setIgnoreCursorEvents(false);
            window.setFocus();
        }
    });

    // Tool control
    createEffect(async () => {
        const onMouseDown = (event: MouseEvent) => {
            tool().down();
            tool().move(event.clientX, event.clientY);
        };

        const onMouseMove = (event: MouseEvent) => {
            if (event.buttons === 1) {
                tool().move(event.clientX, event.clientY);
            }
        };

        const onMouseUp = () => {
            tool().up();
        };

        addEventListener("mousedown", onMouseDown);
        addEventListener("mousemove", onMouseMove);
        addEventListener("mouseup", onMouseUp);

        onCleanup(() => {
            removeEventListener("mousedown", onMouseDown);
            removeEventListener("mousemove", onMouseMove);
            removeEventListener("mouseup", onMouseUp);
        });
    });

    return (
        <canvas
            class="w-screen h-screen cursor-crosshair"
            ref={canvasElement}
        />
    );
}
