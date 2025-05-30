class Pen {
    private beforePainted: [number, number] | undefined = undefined;

    constructor(
        private readonly ctx: CanvasRenderingContext2D,
        public color = "blue",
        public size = 10,
    ) {}

    down() {
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

export class Canvas {
    private readonly ctx: CanvasRenderingContext2D;
    public readonly pen: Pen;

    constructor(canvas: HTMLCanvasElement) {
        this.ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

        this.pen = new Pen(this.ctx);
    }

    getContext(): CanvasRenderingContext2D {
        return this.ctx;
    }
}
