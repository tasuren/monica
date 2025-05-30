import { listen } from "@tauri-apps/api/event";
import { createEffect, onCleanup } from "solid-js";
import { Canvas } from "./canvas";
import "./DrawApp.css";

function DrawApp() {
    let canvas: Canvas | undefined = undefined;

    createEffect(async () => {
        const unListen = await listen("mouse-drag", (event) => {
            if (!canvas) return;
            const { x, y } = event.payload as { x: number; y: number };
            canvas.paint(x, y, { color: "black" });
        });

        onCleanup(unListen);
    });

    return (
        <canvas
            class="w-screen h-screen"
            ref={(element) => {
                canvas = new Canvas(element);
            }}
        />
    );
}

export default DrawApp;
