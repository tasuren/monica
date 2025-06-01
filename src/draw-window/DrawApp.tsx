import { CanvasControllerProvider } from "./CanvasController";
import { CanvasArea } from "./components/CanvasArea";
import "./DrawApp.css";

function DrawApp() {
    return (
        <div class="overflow-hidden select-none">
            <CanvasControllerProvider>
                <CanvasArea />
            </CanvasControllerProvider>
        </div>
    );
}

export default DrawApp;
