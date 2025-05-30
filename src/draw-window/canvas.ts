class Pen {
    constructor(
        private readonly ctx: CanvasRenderingContext2D,
        public size = 10,
    ) {}
}

export class Canvas {
    private readonly canvas: HTMLCanvasElement;
    private readonly ctx: CanvasRenderingContext2D;

    constructor(canvas: HTMLCanvasElement) {
        this.canvas = canvas;
        this.ctx = canvas.getContext("2d") as CanvasRenderingContext2D;

        window.addEventListener("resize", () => this.resize());
    }

    private resize() {
        this.canvas.width = window.innerWidth;
        this.canvas.height = window.innerHeight;
    }

    getContext(): CanvasRenderingContext2D {
        return this.ctx;
    }

    paint(x: number, y: number, opts: { color: string; size?: number }) {
        this.ctx.fillStyle = opts.color;

        const size = opts.size ?? 10;
        this.ctx.fillRect(x, y, size, size);
    }
}
