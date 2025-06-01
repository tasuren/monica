import {
    isRegistered,
    register,
    unregister,
} from "@tauri-apps/plugin-global-shortcut";
import Eraser from "lucide-solid/icons/eraser";
import MousePointer2 from "lucide-solid/icons/mouse-pointer-2";
import Pencil from "lucide-solid/icons/pencil";
import Trash2 from "lucide-solid/icons/trash-2";
import {
    type ParentProps,
    createEffect,
    createMemo,
    onCleanup,
} from "solid-js";
import type { ToolKind } from "../../common/tool";
import { cl } from "../../utils";
import { useCanvas, useTool } from "../CanvasState";

export function Controller() {
    return (
        <div class="flex justify-between">
            <div class="flex gap-2">
                <ToolButton tool="cursor" shortcutKey="Escape">
                    <MousePointer2 color="white" />
                </ToolButton>

                <ToolButton tool="pen">
                    <Pencil color="white" />
                </ToolButton>

                <ToolButton tool="eraser">
                    <Eraser color="white" />
                </ToolButton>
            </div>

            <ResetButton />
        </div>
    );
}

function ToolButton(
    props: ParentProps<{ tool: ToolKind; shortcutKey?: string }>,
) {
    const [tool, setTool] = useTool();
    const isSelected = createMemo(() => tool() === props.tool);

    const onClick = () => {
        setTool(props.tool);
    };

    if (props.shortcutKey) {
        const shortcutKey = props.shortcutKey as string;

        createEffect(async () => {
            if (isSelected()) {
                await unregister(shortcutKey);
            } else {
                await register(shortcutKey, onClick);
            }

            onCleanup(async () => {
                if (await isRegistered(shortcutKey)) {
                    await unregister(shortcutKey);
                }
            });
        });
    }

    return (
        <button
            type="button"
            class={cl(
                "rounded-xl p-2",
                isSelected()
                    ? "bg-white/40"
                    : "hover:bg-white/20 cursor-pointer",
            )}
            onClick={onClick}
        >
            {props.children}
        </button>
    );
}

export function ResetButton() {
    const canvas = useCanvas();

    return (
        <button
            type="button"
            class="p-2 cursor-pointer"
            onClick={canvas.clear.bind(canvas)}
        >
            <Trash2 color="red" />
        </button>
    );
}
