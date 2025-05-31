import "./DrawApp.css";
import { GlobalStateProvider } from "./GlobalState";
import { CanvasArea } from "./components/CanvasArea";
import { Tooltip } from "./components/Tooltip";

function DrawApp() {
    return (
        <GlobalStateProvider>
            <Tooltip />
            <CanvasArea />
        </GlobalStateProvider>
    );
}

export default DrawApp;
