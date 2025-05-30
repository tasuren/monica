export const TOOLS = ["cursor", "pen", "eraser"] as const;
export type Tool = (typeof TOOLS)[number];

class Pen {
    private beforePainted: [number, number] | undefined = undefined;

    constructor(
        private readonly ctx: CanvasRenderingContext2D,
        public color = "blue",
        public size = 10,
    ) {}

    down() {
        this.ctx.globalCompositeOperation = "source-over";
        this.ctx.fillStyle = this.color;
    }

    paint(x: number, y: number) {
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

class Eraser {
    private beforeErased: [number, number] | undefined = undefined;

    constructor(
        private readonly ctx: CanvasRenderingContext2D,
        public size = 10,
    ) {}

    down() {
        this.ctx.globalCompositeOperation = "destination-out";
    }

    erase(x: number, y: number) {
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
        console.log(1, this.ctx.globalCompositeOperation);
    }
}

export class Canvas {
    private readonly ctx: CanvasRenderingContext2D;

    public readonly pen: Pen;
    public readonly eraser: Eraser;

    constructor(canvas: HTMLCanvasElement) {
        this.ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

        this.pen = new Pen(this.ctx);
        this.eraser = new Eraser(this.ctx);
    }

    getContext(): CanvasRenderingContext2D {
        return this.ctx;
    }
}
