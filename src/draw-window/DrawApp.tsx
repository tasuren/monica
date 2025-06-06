import { getCurrentWindow } from "@tauri-apps/api/window";
import { createEffect, onCleanup } from "solid-js";
import {
    CanvasControllerProvider,
    useDrawing,
    useTool,
} from "./CanvasController";
import { CanvasArea } from "./components/CanvasArea";
import "./DrawApp.css";
import { setupMouseToolGlue } from "./lib/mouse-tool-glue";

function App() {
    const [tool] = useTool();
    const drawing = useDrawing();

    const window = getCurrentWindow();
    let element!: HTMLDivElement;

    createEffect(async () => {
        if (drawing()) {
            await window.setIgnoreCursorEvents(false);
        } else {
            await window.setIgnoreCursorEvents(true);
        }
    });

    createEffect(() => {
        const cleanup = setupMouseToolGlue(tool);

        onCleanup(() => cleanup());
    });

    return (
        <div class="overflow-hidden select-none" ref={element}>
            <CanvasArea />
        </div>
    );
}

function DrawApp() {
    return (
        <CanvasControllerProvider>
            <App />
        </CanvasControllerProvider>
    );
}

export default DrawApp;
