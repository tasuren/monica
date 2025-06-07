import { platform } from "@tauri-apps/plugin-os";
import { createEffect, onCleanup, onMount } from "solid-js";
import { setupMouseEventHandler } from "../../common/mouse";
import { useCanvas, useDrawing, useTool } from "../CanvasController";
import { Canvas } from "../lib/canvas";

export function CanvasArea() {
    let canvasElement!: HTMLCanvasElement;
    const [_, setCanvas] = useCanvas();

    onMount(() => {
        canvasElement.width = window.innerWidth;
        canvasElement.height = window.innerHeight;

        setCanvas(new Canvas(canvasElement));
    });

    return (
        <>
            <CircleTool />
            <canvas class="w-screen h-screen" ref={canvasElement} />
        </>
    );
}

function CircleTool() {
    const [tool] = useTool();
    const [canvas] = useCanvas();
    const drawing = useDrawing();
    let element!: HTMLDivElement;

    // Initilization
    createEffect(() => {
        canvas().circle.setElement(element);
    });

    // Cicle visibility
    createEffect(() => {
        element.style.display =
            !drawing() && tool().kind === "circle" ? "block" : "none";
    });

    // Circle movement
    createEffect(async () => {
        const cleanup = await setupMouseEventHandler({
            onMouseMove(x, y) {
                const circle = canvas().circle;
                
                if (platform() === "windows") {
                    // Possibly, cursor offset differs on Windows
                    circle.move(x - 5, y);
                } else {
                    circle.move(x, y);
                }
            },
        });

        onCleanup(() => cleanup());
    });

    return (
        <div
            class="bg-red-600/40 w-10 h-10 rounded-full absolute"
            ref={element}
        >
            {"　"}
        </div>
    );
}
