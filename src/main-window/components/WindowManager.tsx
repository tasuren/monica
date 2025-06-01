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

        const onMouseMove = async (x: number, y: number) => {
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
                await setLock(false);
            }
        };

        const cleanup = await setupMouseEventHandler({
            onMouseMove,
        });

        onCleanup(() => {
            cleanup();
            unListenMove();
            unListenResize();
        });
    });

    return props.children;
}
