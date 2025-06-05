import { createEffect, onCleanup } from "solid-js";
import { CanvasProvider, useLock, useTool } from "./CanvasState";
import { Controller } from "./components/Controller";
import { setupWindowManagement } from "./lib/window-management";
import "./MainApp.css";

function App() {
    const [lock, setLock] = useLock();
    const [tool, _] = useTool();

    createEffect(async () => {
        const cleanup = await setupWindowManagement({
            tool() {
                return tool().kind;
            },
            lock,
            setLock,
        });

        onCleanup(() => cleanup());
    });

    return (
        <div
            class="w-screen h-screen p-2 flex flex-col justify-center"
            data-tauri-drag-region
        >
            <Controller />
        </div>
    );
}

function MainApp() {
    return (
        <CanvasProvider>
            <App />
        </CanvasProvider>
    );
}

export default MainApp;
