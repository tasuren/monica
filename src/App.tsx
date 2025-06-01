import { getCurrentWindow } from "@tauri-apps/api/window";
import DrawApp from "./draw-window/DrawApp";
import MainApp from "./main-window/MainApp";

function App() {
    const isMainWindow = getCurrentWindow().label === "main";

    if (isMainWindow) {
        return <MainApp />;
    }

    return <DrawApp />;
}

export default App;
