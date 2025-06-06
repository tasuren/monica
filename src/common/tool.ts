export const TOOL_KINDS = ["cursor", "pen", "eraser", "circle"] as const;
export type ToolKind = (typeof TOOL_KINDS)[number];
