import { getCurrentWindow } from "@tauri-apps/api/window";
import {
    type ParentProps,
    createEffect,
    createMemo,
    onCleanup,
} from "solid-js";
import { useCanvas, useLock, useTool } from "../CanvasController";

export function WindowManager(props: ParentProps) {
    // Tool control
    createEffect(async () => {
    });

    return props.children;
}
