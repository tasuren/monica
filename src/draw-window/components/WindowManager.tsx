import { getCurrentWindow } from "@tauri-apps/api/window";
import {
    type ParentProps,
    createEffect,
    createMemo,
    onCleanup,
} from "solid-js";
import { useCanvas, useLock, useTool } from "../CanvasController";

export function WindowManager(props: ParentProps) {
    const [tool] = useTool();
    const [lock] = useLock();
    const [canvas] = useCanvas();
    const notDrawing = createMemo(() => tool().kind === "cursor" || lock());

    const window = getCurrentWindow();

    createEffect(async () => {
        const canvas_ = canvas();
        if (!canvas_) return;

        if (notDrawing()) {
            window.setIgnoreCursorEvents(true);
        } else {
            window.setIgnoreCursorEvents(false);
        }
    });

    // Tool control
    createEffect(async () => {
        const onMouseDown = (event: MouseEvent) => {
            tool().down();
            tool().move(event.clientX, event.clientY);
        };

        const onMouseMove = (event: MouseEvent) => {
            if (event.buttons === 1) {
                tool().move(event.clientX, event.clientY);
            }
        };

        const onMouseUp = () => {
            tool().up();
        };

        addEventListener("mousedown", onMouseDown);
        addEventListener("mousemove", onMouseMove);
        addEventListener("mouseup", onMouseUp);

        onCleanup(() => {
            removeEventListener("mousedown", onMouseDown);
            removeEventListener("mousemove", onMouseMove);
            removeEventListener("mouseup", onMouseUp);
        });
    });

    return props.children;
}
