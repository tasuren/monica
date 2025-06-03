import { getCurrentWindow } from "@tauri-apps/api/window";
import {
    isRegistered,
    register,
    unregister,
} from "@tauri-apps/plugin-global-shortcut";
import Eraser from "lucide-solid/icons/eraser";
import Grip from "lucide-solid/icons/grip";
import MousePointer2 from "lucide-solid/icons/mouse-pointer-2";
import Pencil from "lucide-solid/icons/pencil";
import Trash2 from "lucide-solid/icons/trash-2";
import X from "lucide-solid/icons/x";
import {
    type ParentProps,
    createEffect,
    createMemo,
    onCleanup,
} from "solid-js";
import type { ToolKind } from "../../common/tool";
import { cl } from "../../common/utils";
import { useCanvas, useTool } from "../CanvasState";

export function Controller() {
    return (
        <div class="flex justify-between">
            <div class="flex gap-2">
                <div class="flex justify-center p-2" data-tauri-drag-region>
                    <Grip
                        class="stroke-black dark:stroke-white"
                        data-tauri-drag-region
                    />
                </div>

                <ToolButton tool="cursor" shortcutKey="Escape">
                    <MousePointer2 class="stroke-black dark:stroke-white" />
                </ToolButton>

                <ToolButton tool="pen">
                    <Pencil class="stroke-black dark:stroke-white" />
                </ToolButton>

                <ToolButton tool="eraser">
                    <Eraser class="stroke-black dark:stroke-white" />
                </ToolButton>
            </div>

            <div class="flex gap-2">
                <ResetButton />

                <ExitButton />
            </div>
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

function ResetButton() {
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

function ExitButton() {
    const onClick = () => {
        getCurrentWindow().close();
    };

    return (
        <button type="button" class="p-2 cursor-pointer" onClick={onClick}>
            <X class="stroke-black dark:stroke-white" />
        </button>
    );
}
