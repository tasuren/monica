import { CanvasProvider } from "./CanvasState";
import { Controller } from "./components/Controller";
import "./MainApp.css";

function MainApp() {
    return (
        <div class="w-screen h-screen bg-black p-2">
            <CanvasProvider>
                <Controller />
            </CanvasProvider>
        </div>
    );
}

export default MainApp;
