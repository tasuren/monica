import { createEffect, onCleanup } from "solid-js";
import { CanvasProvider, useLock, useTool } from "./CanvasState";
import { Controller } from "./components/Controller";
import { setupWindowManagement } from "./lib/window-management";
import "./MainApp.css";

function App() {
    const [lock, setLock] = useLock();
    const [tool, _] = useTool();

    createEffect(async () => {
        const cleanup = await setupWindowManagement({ tool, lock, setLock });

        onCleanup(() => cleanup());
    });

    return (
        <div class="w-screen h-screen bg-black p-2">
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
