import { createEffect, onCleanup } from "solid-js";
import { CanvasProvider, useLock } from "./CanvasState";
import { Controller } from "./components/Controller";
import { setupWindowManagement } from "./lib/window-management";
import "./MainApp.css";

function App() {
    const [lock, setLock] = useLock();

    createEffect(async () => {
        const cleanup = await setupWindowManagement({ lock, setLock });

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
