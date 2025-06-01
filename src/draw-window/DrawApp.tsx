import "./DrawApp.css";
import { GlobalStateProvider } from "./GlobalState";
import { CanvasArea } from "./components/CanvasArea";
import { Tooltip } from "./components/Tooltip";

function DrawApp() {
    return (
        <div class="overflow-hidden select-none">
            <GlobalStateProvider>
                <Tooltip />
                <CanvasArea />
            </GlobalStateProvider>
        </div>
    );
}

export default DrawApp;
