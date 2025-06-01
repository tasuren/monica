export function cl(
    ...classNames: (string | null | undefined | false)[]
): string {
    return classNames.filter(Boolean).join(" ");
}
