import type { Tool } from "./tool";

export function setupMouseToolGlue(tool: () => Tool) {
    const onMouseDown = (event: MouseEvent) => {
        tool().down();
        tool().move(event.clientX, event.clientY);
    };

    const onMouseMove = (event: MouseEvent) => {
        if (event.buttons === 1) {
            if (!tool().isDowned()) {
                tool().down();
            }

            tool().move(event.clientX, event.clientY);
        }
    };

    const onMouseUp = () => {
        tool().up();
    };

    addEventListener("mousedown", onMouseDown);
    addEventListener("mousemove", onMouseMove);
    addEventListener("mouseup", onMouseUp);

    return () => {
        removeEventListener("mousedown", onMouseDown);
        removeEventListener("mousemove", onMouseMove);
        removeEventListener("mouseup", onMouseUp);
    };
}
