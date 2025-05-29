import { getCurrentWindow } from "@tauri-apps/api/window";
import DrawApp from "./draw-window/DrawApp";
import HomeApp from "./home-window/HomeApp";

function App() {
  const isDrawWindow = getCurrentWindow().label.startsWith("draw-window");

  if (isDrawWindow) {
    return <DrawApp />;
  }

  return <HomeApp />;
}

export default App;
