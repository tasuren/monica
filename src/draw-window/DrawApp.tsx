import { listen } from "@tauri-apps/api/event";
import { createEffect, onCleanup } from "solid-js";
import { Canvas } from "./canvas";
import "./DrawApp.css";

function DrawApp() {
    let canvas: Canvas | undefined = undefined;

    createEffect(async () => {
        const unListenMouseDown = await listen("mouse-down", (event) => {
            if (!canvas) return;

            const [x, y] = event.payload as [number, number];

            canvas.pen.down();
            canvas.pen.paint(x, y);
        });

        const unListenMouseMove = await listen("mouse-move", (event) => {
            if (!canvas) return;

            const [x, y] = event.payload as [number, number];
            canvas.pen.paint(x, y);
        });

        const unListenMouseUp = await listen("mouse-up", () => {
            if (!canvas) return;

            canvas.pen.up();
        });

        onCleanup(() => {
            unListenMouseDown();
            unListenMouseMove();
            unListenMouseUp();
        });
    });

    return (
        <canvas
            class="w-screen h-screen"
            ref={(element) => {
                element.width = window.innerWidth;
                element.height = window.innerHeight;

                canvas = new Canvas(element);
            }}
        />
    );
}

export default DrawApp;
