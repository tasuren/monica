import { type Window, getCurrentWindow } from "@tauri-apps/api/window";
import { type ParentProps, createEffect, onCleanup } from "solid-js";
import { setupMouseEventHandler } from "../../common/mouse";
import { useLock } from "../CanvasState";

async function getWindowSize(window: Window, scaleFactor: number) {
    return (await window.outerSize()).toLogical(scaleFactor);
}

async function getWindowPosition(window: Window, scaleFactor: number) {
    return (await window.outerPosition()).toLogical(scaleFactor);
}

export function WindowManager(props: ParentProps) {
    const [lock, setLock] = useLock();

    createEffect(async () => {
        const window = getCurrentWindow();

        const windowScaleFactor = await window.scaleFactor();
        let position = await getWindowPosition(window, windowScaleFactor);
        let size = await getWindowSize(window, windowScaleFactor);

        const unListenResize = await window.onResized(
            ({ payload: newSize }) => {
                size = newSize.toLogical(windowScaleFactor);
            },
        );

        const unListenMove = await window.onMoved(
            ({ payload: newPosition }) => {
                position = newPosition.toLogical(windowScaleFactor);
            },
        );

        let dragging = false;
        const unListenMoved = await window.onMoved(() => {
            // When the window is dragging, we want to lock the draw window.
            dragging = true;
            console.log("dragged");
            setLock(true);
        });

        const onMouseDown = () => {
            dragging = true;
        };

        const onMouseMove = async (x: number, y: number) => {
            if (dragging) return;

            const isInside =
                x >= position.x &&
                x <= position.x + size.width &&
                y >= position.y &&
                y <= position.y + size.height;

            if (isInside) {
                if (lock()) return;

                await setLock(true);
                window.setFocus();
            } else if (lock()) {
                console.log("lock released");
                await setLock(false);
            }
        };

        const onMouseUp = () => {
            if (dragging) {
                dragging = false;
            }
        };

        const cleanup = await setupMouseEventHandler({
            onMouseDown,
            onMouseMove,
            onMouseUp,
        });

        onCleanup(() => {
            cleanup();
            unListenMove();
            unListenResize();
            unListenMoved();
        });
    });

    return props.children;
}
