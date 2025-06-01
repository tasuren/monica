import type { ToolKind } from "../../common/tool";

export abstract class Tool {
    abstract get kind(): ToolKind;

    abstract down(): void;
    abstract move(x: number, y: number): void;
    abstract up(): void;
}

export class Cursor extends Tool {
    public readonly kind: ToolKind = "cursor";

    down(): void {}
    move(_x: number, _y: number): void {}
    up(): void {}
}

export class Pen extends Tool {
    public readonly kind: ToolKind = "pen";

    private beforePainted: [number, number] | undefined = undefined;

    constructor(
        private readonly ctx: CanvasRenderingContext2D,
        public color = "blue",
        public size = 10,
    ) {
        super();
    }

    down() {
        this.ctx.globalCompositeOperation = "source-over";
        this.ctx.fillStyle = this.color;
    }

    move(x: number, y: number) {
        this.ctx.beginPath();
        this.ctx.arc(x, y, this.size / 2, 0, 2 * Math.PI);
        this.ctx.fill();

        if (this.beforePainted) {
            this.ctx.beginPath();
            this.ctx.moveTo(this.beforePainted[0], this.beforePainted[1]);
            this.ctx.lineTo(x, y);
            this.ctx.strokeStyle = this.color;
            this.ctx.lineWidth = this.size;
            this.ctx.stroke();
        }

        this.beforePainted = [x, y];
    }

    up() {
        this.beforePainted = undefined;
    }
}

export class Eraser extends Tool {
    public readonly kind: ToolKind = "eraser";

    private beforeErased: [number, number] | undefined = undefined;

    constructor(
        private readonly ctx: CanvasRenderingContext2D,
        public size = 10,
    ) {
        super();
    }

    down() {
        this.ctx.globalCompositeOperation = "destination-out";
    }

    move(x: number, y: number) {
        this.ctx.beginPath();
        this.ctx.arc(x, y, this.size / 2, 0, 2 * Math.PI);
        this.ctx.fill();

        if (this.beforeErased) {
            this.ctx.beginPath();
            this.ctx.moveTo(this.beforeErased[0], this.beforeErased[1]);
            this.ctx.lineTo(x, y);
            this.ctx.lineWidth = this.size;
            this.ctx.stroke();
        }

        this.beforeErased = [x, y];
    }

    up() {
        this.beforeErased = undefined;
        this.ctx.globalCompositeOperation = "source-over";
    }
}
