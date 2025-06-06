import { emit } from "@tauri-apps/api/event";
import type { ToolKind } from "../../common/tool";

export class PenContext {
    public readonly kind: ToolKind = "pen";

    setSize(size: number): void {
        emit("set-pen-size", size);
    }

    setColor(color: string): void {
        emit("set-pen-color", color);
    }
}

export class EraserContext {
    public readonly kind: ToolKind = "eraser";

    setSize(size: number): void {
        emit("set-eraser-size", size);
    }
}

export class CursorContext {
    public readonly kind: ToolKind = "cursor";
}

export class CircleContext {
    public readonly kind: ToolKind = "circle";
}

export type ToolContext = PenContext | EraserContext | CursorContext;
export const TOOL_CONTEXTS = {
    pen: new PenContext(),
    eraser: new EraserContext(),
    cursor: new CursorContext(),
    circle: new CircleContext(),
} as const;
