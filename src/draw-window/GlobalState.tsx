import {
    type ParentProps,
    createContext,
    createSignal,
    useContext,
} from "solid-js";
import type { Canvas, Tool } from "./lib/canvas";

export interface GlobalState {
    tool(): Tool;
    setTool(tool: Tool): void;
    canvas(): Canvas;
    setCanvas(canvas: Canvas): void;
    lock(): boolean;
    setLock(lock: boolean): void;
}

export const GlobalStateContext = createContext<GlobalState>();

export function GlobalStateProvider(props: ParentProps) {
    const [tool, setTool] = createSignal<Tool>("cursor");
    const [canvas, setCanvas] = createSignal<Canvas>();
    const [lock, setLock] = createSignal(false);

    return (
        <GlobalStateContext.Provider
            value={{
                tool,
                setTool,
                canvas() {
                    const maybeCanvas = canvas();

                    if (!maybeCanvas)
                        throw new Error("`Canvas` is not set yet.");

                    return maybeCanvas;
                },
                setCanvas,
                lock,
                setLock,
            }}
        >
            {props.children}
        </GlobalStateContext.Provider>
    );
}

export function useGlobalState(): GlobalState {
    const state = useContext(GlobalStateContext);
    if (!state) throw new Error("you must use `GlobalStateProvider`.");

    return state;
}

export function useTool() {
    const state = useGlobalState();
    return [state.tool, state.setTool] as const;
}

export function useCanvas() {
    const state = useGlobalState();
    return [state.canvas, state.setCanvas] as const;
}

export function useLock() {
    const state = useGlobalState();
    return [state.lock, state.setLock] as const;
}
