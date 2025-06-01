import {
    type ParentProps,
    createContext,
    createEffect,
    createSignal,
    useContext,
} from "solid-js";
import type { ToolKind } from "../common/tool";
import { CanvasHandle } from "./lib/canvas-handle";

export interface CanvasState {
    tool(): ToolKind;
    setTool(tool: ToolKind): void;
    clear(): void;
}

export const CanvasContext = createContext<CanvasState>();

export function CanvasProvider(props: ParentProps) {
    const [tool, setTool] = createSignal<ToolKind>("cursor");
    const canvasHandle = new CanvasHandle();

    // Canvas management
    createEffect(() => {
        canvasHandle.setTool(tool());
    });

    const clear = () => canvasHandle.clear();

    return (
        <CanvasContext.Provider
            value={{
                tool,
                setTool,
                clear,
            }}
        >
            {props.children}
        </CanvasContext.Provider>
    );
}

export function useCanvas(): CanvasState {
    const state = useContext(CanvasContext);
    if (!state) throw new Error("you must use `CanvasProvider`.");

    return state;
}

export function useTool() {
    const state = useCanvas();
    return [state.tool, state.setTool] as const;
}
