export function cl(
    ...classNames: (string | null | undefined | false)[]
): string {
    return classNames.filter(Boolean).join(" ");
}

export function isInsideRect(
    x: number,
    y: number,
    position: { x: number; y: number },
    size: { width: number; height: number },
): boolean {
    return (
        x >= position.x &&
        x <= position.x + size.width &&
        y >= position.y &&
        y <= position.y + size.height
    );
}
