import type { Canvas, Tool } from "./canvas";

export class ToolController {
    constructor(
        private readonly canvas: Canvas,
        private readonly getCurrentTool: () => Tool,
    ) {}

    onMouseDown(x: number, y: number) {
        const tool = this.getCurrentTool();

        switch (tool) {
            case "pen":
                this.canvas.pen.down();
                this.canvas.pen.paint(x, y);
                break;
            case "eraser":
                this.canvas.eraser.down();
                this.canvas.eraser.erase(x, y);
                break;
        }
    }

    onMouseMoveWithDown(x: number, y: number) {
        const tool = this.getCurrentTool();

        switch (tool) {
            case "pen":
                this.canvas.pen.paint(x, y);
                break;
            case "eraser":
                this.canvas.eraser.erase(x, y);
                break;
        }
    }

    onMouseUp() {
        const tool = this.getCurrentTool();

        switch (tool) {
            case "pen":
                this.canvas.pen.up();
                break;
            case "eraser":
                this.canvas.eraser.up();
                break;
        }
    }
}
