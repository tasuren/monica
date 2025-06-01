import { CanvasProvider } from "./CanvasState";
import { Controller } from "./components/Controller";
import { WindowManager } from "./components/WindowManager";
import "./MainApp.css";

function MainApp() {
    return (
        <CanvasProvider>
            <WindowManager>
                <div class="w-screen h-screen bg-black p-2">
                    <Controller />
                </div>
            </WindowManager>
        </CanvasProvider>
    );
}

export default MainApp;
