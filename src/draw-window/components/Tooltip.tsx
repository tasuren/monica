import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import Eraser from "lucide-solid/icons/eraser";
import Grip from "lucide-solid/icons/grip";
import MousePointer2 from "lucide-solid/icons/mouse-pointer-2";
import Pencil from "lucide-solid/icons/pencil";
import {
    type ParentProps,
    createEffect,
    createSignal,
    onCleanup,
} from "solid-js";
import { cl } from "../../utils";
import { useTool } from "../GlobalState";
import type { Tool } from "../lib/canvas";

function ToolButton(props: ParentProps<{ tool: Tool }>) {
    const [tool, setTool] = useTool();

    return (
        <button
            type="button"
            class={cl(
                "rounded-xl",
                "hover:bg-white/20",
                "p-2",
                tool() === props.tool && "bg-white/40",
            )}
            onClick={() => setTool(props.tool)}
        >
            {props.children}
        </button>
    );
}

export function Tooltip() {
    // This tooltip is used to show the tools available in the draw window.
    // It also listens to mouse movements to determine if the mouse is inside the tooltip
    // and catch the cursor events because the draw window always ignores cursor events.

    let tooltipElement!: HTMLDivElement;
    let isMouseInside = false;
    const [rect, setRect] = createSignal<DOMRect>();
    const [tool, _] = useTool();

    createEffect(async () => {
        const unListenMouseMove = await listen("mouse-move", (event) => {
            const [x, y, _] = event.payload as [number, number, boolean];
            const tooltipRect = tooltipElement.getBoundingClientRect();
            setRect(tooltipRect);
            const window = getCurrentWindow();

            if (
                x >= tooltipRect.left &&
                x <= tooltipRect.right &&
                y >= tooltipRect.top &&
                y <= tooltipRect.bottom
            ) {
                if (isMouseInside) return;

                window.setFocus();
                window.setIgnoreCursorEvents(false);
                isMouseInside = true;
            } else if (isMouseInside) {
                if (tool() === "cursor") {
                    console.log(1, x, y, tooltipRect);
                    window.setIgnoreCursorEvents(true);
                }

                isMouseInside = false;
            }
        });

        onCleanup(() => {
            unListenMouseMove();
        });
    });

    const [position, setPosition] = createSignal({ x: 60, y: 80 });

    return (
        <div
            class={cl(
                "rounded-xl",
                "border border-white/40 bg-black/80",
                "p-4",
                "flex flex-col gap-2",
            )}
            style={{
                position: "absolute",
                left: `${position().x}px`,
                top: `${position().y}px`,
            }}
            ref={tooltipElement}
        >
            <ToolButton tool="cursor">
                <MousePointer2 color="white" />
            </ToolButton>
            <ToolButton tool="pen">
                <Pencil color="white" />
            </ToolButton>
            <ToolButton tool="eraser">
                <Eraser color="white" />
            </ToolButton>

            <TooltipGrip
                tooltipRect={rect}
                setPosition={(x, y) => setPosition({ x, y })}
            />
        </div>
    );
}

function TooltipGrip(props: {
    tooltipRect: () => DOMRect | undefined;
    setPosition: (x: number, y: number) => void;
}) {
    const { tooltipRect, setPosition } = props;

    let relativePosition: { x: number; y: number } | undefined = undefined;
    let position = { x: 0, y: 0 };

    const onMouseDown = () => {
        const rect = tooltipRect();
        if (!rect) return;

        relativePosition = {
            x: position.x - rect.left,
            y: position.y - rect.top,
        };
    };
    const onMouseUp = () => {
        relativePosition = undefined;
    };

    createEffect(async () => {
        const unListen = await listen("mouse-move", (event) => {
            const [x, y, _] = event.payload as [number, number, boolean];
            position = { x, y };

            if (relativePosition) {
                const { x: relativeX, y: relativeY } = relativePosition;

                setPosition(x - relativeX, y - relativeY);
            }
        });

        onCleanup(() => {
            unListen();
        });
    });

    return (
        <div
            class="flex justify-center p-2"
            onMouseDown={onMouseDown}
            onMouseUp={onMouseUp}
        >
            <Grip color="white" class="cursor-grab focus:cursor-grabbing" />
        </div>
    );
}
