import { listen } from "@tauri-apps/api/event";
import type { ToolKind } from "../../common/tool";
import type { Canvas } from "./canvas";

export async function setupEventHandler(
    canvas: Canvas,
    opts: {
        setTool: (tool: ToolKind) => void;
        setLock: (lock: boolean) => void;
    },
): Promise<() => void> {
    const unListenCanvasClear = await listen("canvas-clear", async () => {
        canvas.clear();
    });

    const unListenSetTool = await listen("canvas-set-tool", async (event) => {
        opts.setTool(event.payload as ToolKind);
    });

    const unListenSetLock = await listen("canvas-set-lock", async (event) => {
        opts.setLock(event.payload as boolean);
    });

    const cleanup = () => {
        unListenCanvasClear();
        unListenSetTool();
        unListenSetLock();
    };

    return cleanup;
}
