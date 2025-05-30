import {
    type ParentProps,
    createContext,
    createSignal,
    useContext,
} from "solid-js";
import type { Tool } from "./lib/canvas";

export interface GlobalState {
    tool(): Tool;
    setTool(tool: Tool): void;
}

export const GlobalStateContext = createContext<GlobalState>();

export function GlobalStateProvider(props: ParentProps) {
    const [tool, setTool] = createSignal<Tool>("cursor");

    return (
        <GlobalStateContext.Provider value={{ tool, setTool }}>
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
