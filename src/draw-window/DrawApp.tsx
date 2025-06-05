import { getCurrentWindow } from "@tauri-apps/api/window";
import { createEffect, createMemo, onCleanup } from "solid-js";
import {
    CanvasControllerProvider,
    useLock,
    useTool,
} from "./CanvasController";
import { CanvasArea } from "./components/CanvasArea";
import { CursorDecoration } from "./components/CursorDecoration";
import "./DrawApp.css";
import { setupMouseToolGlue } from "./lib/mouse-tool-glue";

function App() {
    const [tool] = useTool();
    const [lock] = useLock();
    const drawing = createMemo(() => tool().kind !== "cursor" && !lock());

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
            <CursorDecoration drawing={drawing} />
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
