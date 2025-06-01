import type { ToolKind } from "../../common/tool";
import { Cursor, Eraser, Pen, type Tool } from "./tool";

export class Canvas {
    private readonly ctx: CanvasRenderingContext2D;
    private readonly element: HTMLCanvasElement;

    public readonly cursor: Cursor;
    public readonly pen: Pen;
    public readonly eraser: Eraser;

    constructor(canvasElement: HTMLCanvasElement) {
        this.ctx = canvasElement.getContext("2d") as CanvasRenderingContext2D;
        this.element = canvasElement;

        this.cursor = new Cursor();
        this.pen = new Pen(this.ctx);
        this.eraser = new Eraser(this.ctx);
    }

    getElement(): HTMLCanvasElement {
        return this.element;
    }

    getContext(): CanvasRenderingContext2D {
        return this.ctx;
    }

    getTool(kind: ToolKind): Tool {
        switch (kind) {
            case "cursor":
                return this.cursor;
            case "pen":
                return this.pen;
            case "eraser":
                return this.eraser;
            default:
                throw new Error(`Unknown tool kind: ${kind}`);
        }
    }

    clear() {
        this.ctx.globalCompositeOperation = "destination-out";
        this.ctx.fillRect(0, 0, this.ctx.canvas.width, this.ctx.canvas.height);
        this.ctx.globalCompositeOperation = "source-over";
    }
}
