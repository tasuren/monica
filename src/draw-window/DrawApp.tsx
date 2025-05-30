import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { createEffect, onCleanup } from "solid-js";
import "./DrawApp.css";
import { GlobalStateProvider, useTool } from "./GlobalState";
import { Tooltip } from "./components/Tooltip";
import { Canvas } from "./lib/canvas";
import { ToolController } from "./lib/tool-controller";

function CanvasArea() {
    let canvas!: Canvas;
    const [tool, _] = useTool();

    createEffect(() => {
        const window = getCurrentWindow();

        if (tool() === "cursor") {
            window.setIgnoreCursorEvents(true);
        } else {
            window.setIgnoreCursorEvents(false);
        }
    });

    createEffect(async () => {
        const tools = new ToolController(canvas, tool);

        const unListenMouseDown = await listen("mouse-down", (event) => {
            const [x, y, _] = event.payload as [number, number, boolean];

            tools.onMouseDown(x, y);
        });

        const unListenMouseMove = await listen("mouse-move", (event) => {
            const [x, y, isMouseDown] = event.payload as [
                number,
                number,
                boolean,
            ];

            if (!isMouseDown) return;
            tools.onMouseMoveWithDown(x, y);
        });

        const unListenMouseUp = await listen("mouse-up", () => {
            tools.onMouseUp();
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

function DrawApp() {
    return (
        <GlobalStateProvider>
            <Tooltip />
            <CanvasArea />
        </GlobalStateProvider>
    );
}

export default DrawApp;
