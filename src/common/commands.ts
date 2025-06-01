import { invoke } from "@tauri-apps/api/core";

export async function getMousePosition(): Promise<{ x: number; y: number }> {
    const [x, y] = (await invoke("get_mouse_position")) as [number, number];
    return { x, y };
}
