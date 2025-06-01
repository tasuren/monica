import {
    type LogicalPosition,
    type LogicalSize,
    type Window,
    getCurrentWindow,
} from "@tauri-apps/api/window";
import { getMousePosition } from "../../common/commands";
import {
    getWindowPosition,
    getWindowSize,
    isInsideRect,
} from "./window-rect-utils";

export class WindowState {
    public readonly window: Window = getCurrentWindow();

    constructor(private readonly rectTracker: WindowRectTracker) {}

    async getPosition() {
        return await this.rectTracker.getPosition();
    }

    async getSize() {
        return await this.rectTracker.getSize();
    }

    async isInside(mousePosition?: { x: number; y: number }): Promise<boolean> {
        const { x, y } = mousePosition ?? (await getMousePosition());

        return isInsideRect(
            x,
            y,
            await this.getPosition(),
            await this.getSize(),
        );
    }
}

export class WindowRectTracker {
    private readonly window = getCurrentWindow();
    private size: LogicalSize | undefined = undefined;
    private position: LogicalPosition | undefined = undefined;
    private cleanup = () => {};

    async listen() {
        const unListenResized = await this.window.onResized(
            async ({ payload: size }) => {
                this.size = size.toLogical(await this.window.scaleFactor());
            },
        );

        const unListenMoved = await this.window.onMoved(
            async ({ payload: position }) => {
                this.position = position.toLogical(
                    await this.window.scaleFactor(),
                );
            },
        );

        this.cleanup = () => {
            unListenResized();
            unListenMoved();
        };
    }

    async getPosition(): Promise<LogicalPosition> {
        if (this.position === undefined) {
            this.position = await getWindowPosition(this.window);
        }

        return this.position;
    }

    async getSize(): Promise<LogicalSize> {
        if (this.size === undefined) {
            this.size = await getWindowSize(this.window);
        }

        return this.size;
    }

    async unListen() {
        this.cleanup();
    }
}
