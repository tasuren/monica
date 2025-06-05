import Eraser from "lucide-solid/icons/eraser";
import Pencil from "lucide-solid/icons/pencil";
import { type JSX, createEffect, createSignal, onCleanup } from "solid-js";
import { useTool } from "../CanvasController";

export function CursorDecoration({ drawing }: { drawing: () => boolean }) {
    let element!: HTMLDivElement;

    createEffect(() => {
        element.style.display = drawing() ? "block" : "none";
    });

    createEffect(() => {
        const onMouseMove = (event: MouseEvent) => {
            element.style.left = `${event.clientX + 20}px`;
            element.style.top = `${event.clientY - 10}px`;
        };

        addEventListener("mousemove", onMouseMove);

        onCleanup(() => {
            removeEventListener("mousemove", onMouseMove);
        });
    });

    const [tool] = useTool();

    const [icon, setIcon] = createSignal<JSX.Element>();
    createEffect(() => {
        switch (tool().kind) {
            case "cursor":
                setIcon(undefined);
                break;
            case "pen":
                setIcon(<Pencil color="white" fill="black" />);
                break;
            case "eraser":
                setIcon(<Eraser color="white" fill="black" />);
                break;
        }
    });

    return (
        <div class="absolute" ref={element}>
            {icon()}
        </div>
    );
}
