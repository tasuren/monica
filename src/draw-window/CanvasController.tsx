import {
    type ParentProps,
    createContext,
    createEffect,
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
    canvas(): Canvas;
    setCanvas(canvas: Canvas): void;
}

export const CanvasControllerContext = createContext<CanvasController>();

export function CanvasControllerProvider(props: ParentProps) {
    const [tool, setTool] = createSignal<Tool>(new Cursor());
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

        const cleanup = await setupEventHandler(canvas_, (toolKind) => {
            setTool(canvas_.getTool(toolKind));
        });

        onCleanup(() => cleanup());
    });

    return (
        <CanvasControllerContext.Provider
            value={{
                tool,
                setTool,
                canvas() {
                    const currentCanvas = canvas();
                    if (!currentCanvas)
                        throw new Error("Canvas is not initialized.");
                    return currentCanvas;
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
