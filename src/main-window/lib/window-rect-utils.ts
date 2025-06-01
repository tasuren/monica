import type {
    LogicalPosition,
    LogicalSize,
    Window,
} from "@tauri-apps/api/window";

export function isInsideRect(
    x: number,
    y: number,
    position: { x: number; y: number },
    size: { width: number; height: number },
): boolean {
    return (
        x >= position.x &&
        x <= position.x + size.width &&
        y >= position.y &&
        y <= position.y + size.height
    );
}

export async function getWindowSize(window: Window): Promise<LogicalSize> {
    const size = await window.outerSize();
    return size.toLogical(await window.scaleFactor());
}

export async function getWindowPosition(
    window: Window,
): Promise<LogicalPosition> {
    const position = await window.outerPosition();
    return position.toLogical(await window.scaleFactor());
}
