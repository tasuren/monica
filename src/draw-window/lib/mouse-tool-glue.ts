import type { Tool } from "./tool";

export function setupMouseToolGlue(tool: () => Tool) {
    const onMouseDown = (event: MouseEvent) => {
        if (tool().kind === "cursor") return;

        tool().down();
        tool().move(event.clientX, event.clientY);
    };

    const onMouseMove = (event: MouseEvent) => {
        if (event.buttons === 1 && tool().kind !== "cursor") {
            if (!tool().isDowned()) {
                tool().down();
            }

            tool().move(event.clientX, event.clientY);
        }
    };

    const onMouseUp = () => {
        if (tool().kind === "cursor") return;

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
