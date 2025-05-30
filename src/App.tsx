import { getCurrentWindow } from "@tauri-apps/api/window";
import DrawApp from "./draw-window/DrawApp";
import HomeApp from "./home-window/HomeApp";

function App() {
    const isMainWindow = getCurrentWindow().label === "main";

    if (isMainWindow) {
        return <HomeApp />;
    }

    return <DrawApp />;
}

export default App;
