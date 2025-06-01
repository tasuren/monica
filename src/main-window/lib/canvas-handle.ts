import { emit } from "@tauri-apps/api/event";

export class CanvasHandle {
    async clear() {
        await emit("canvas-clear");
    }

    async setTool(tool: string) {
        await emit("canvas-set-tool", tool);
    }

    async setLock(lock: boolean) {
        await emit("canvas-set-lock", lock);
    }
}
