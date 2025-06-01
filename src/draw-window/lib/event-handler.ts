import { listen } from "@tauri-apps/api/event";
import type { ToolKind } from "../../common/tool";
import type { Canvas } from "./canvas";

export async function setupEventHandler(
    canvas: Canvas,
    setTool: (tool: ToolKind) => void,
): Promise<() => void> {
    const unListenCanvasClear = await listen("canvas-clear", async () => {
        canvas.clear();
    });

    const unListenSetTool = await listen("canvas-set-tool", async (event) => {
        setTool(event.payload as ToolKind);
    });

    const cleanup = () => {
        unListenCanvasClear();
        unListenSetTool();
    };

    return cleanup;
}
