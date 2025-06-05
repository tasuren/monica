import { onMount } from "solid-js";
import { useCanvas } from "../CanvasController";
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
        <canvas
            class="w-screen h-screen active:cursor-crosshair"
            ref={canvasElement}
        />
    );
}
