import { CanvasControllerProvider } from "./CanvasController";
import { CanvasArea } from "./components/CanvasArea";
import { WindowManager } from "./components/WindowManager";
import "./DrawApp.css";

function DrawApp() {
    return (
        <div class="overflow-hidden select-none cursor-crosshair">
            <CanvasControllerProvider>
                <WindowManager>
                    <CanvasArea />
                </WindowManager>
            </CanvasControllerProvider>
        </div>
    );
}

export default DrawApp;
