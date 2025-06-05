import { getCurrentWindow } from "@tauri-apps/api/window";
import { createEffect, createMemo, onCleanup } from "solid-js";
import {
    CanvasControllerProvider,
    useCanvas,
    useLock,
    useTool,
} from "./CanvasController";
import { CanvasArea } from "./components/CanvasArea";
import { CursorDecoration } from "./components/CursorDecoration";
import "./DrawApp.css";
import { setupMouseToolGlue } from "./lib/mouse-tool-glue";

function App() {
    const [canvas] = useCanvas();
    const [tool] = useTool();
    const [lock] = useLock();
    const drawing = createMemo(() => tool().kind !== "cursor" && !lock());

    const window = getCurrentWindow();

    createEffect(async () => {
        const canvas_ = canvas();
        if (!canvas_) return;

        if (drawing()) {
            await window.setIgnoreCursorEvents(false);
            await window.setFocus()
        } else {
            window.setIgnoreCursorEvents(true);
        }
    });

    createEffect(() => {
        const cleanup = setupMouseToolGlue(tool);

        onCleanup(() => cleanup());
    });

    return (
        <div class="overflow-hidden select-none">
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
