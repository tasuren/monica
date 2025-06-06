import {
    type ParentProps,
    createContext,
    createEffect,
    createMemo,
    createSignal,
    onCleanup,
    useContext,
} from "solid-js";
import type { Canvas } from "./lib/canvas";
import { setupEventHandler } from "./lib/event-handler";
import { Cursor, type Tool } from "./lib/tool";

export interface CanvasController {
    tool(): Tool;
    setTool(tool: Tool): void;
    lock(): boolean;
    setLock(lock: boolean): void;
    drawing(): boolean;
    canvas(): Canvas;
    setCanvas(canvas: Canvas): void;
}

export const CanvasControllerContext = createContext<CanvasController>();

export function CanvasControllerProvider(props: ParentProps) {
    const [tool, setTool] = createSignal<Tool>(new Cursor());
    const [lock, setLock] = createSignal(false);
    const [canvas, setCanvas] = createSignal<Canvas>();

    // Tool state initialization
    let initialized = false;

    createEffect(() => {
        const canvas_ = canvas();

        if (canvas_ && !initialized) {
            initialized = true;
            setTool(canvas_.getTool("cursor"));
        }
    });

    // Event handler setup
    createEffect(async () => {
        const canvas_ = canvas();
        if (!canvas_) return;

        const cleanup = await setupEventHandler(canvas_, {
            setTool: (toolKind) => {
                setTool(canvas_.getTool(toolKind));
            },
            setLock: (lock) => {
                setLock(lock);
            },
        });

        onCleanup(() => cleanup());
    });

    const drawing = createMemo(() => tool().drawTool && !lock());

    return (
        <CanvasControllerContext.Provider
            value={{
                tool,
                setTool,
                lock,
                setLock,
                drawing,
                canvas() {
                    const canvas_ = canvas();
                    if (!canvas_) throw new Error("Canvas is not initialized.");
                    return canvas_;
                },
                setCanvas,
            }}
        >
            {props.children}
        </CanvasControllerContext.Provider>
    );
}

export function useCanvasController(): CanvasController {
    const state = useContext(CanvasControllerContext);
    if (!state) throw new Error("you must use `CanvasProvider`.");

    return state;
}

export function useTool() {
    const state = useCanvasController();
    return [state.tool, state.setTool] as const;
}

export function useCanvas() {
    const state = useCanvasController();
    return [state.canvas, state.setCanvas] as const;
}

export function useLock() {
    const state = useCanvasController();
    return [state.lock, state.setLock] as const;
}

export function useDrawing() {
    const state = useCanvasController();
    return state.drawing;
}
