export function fileSize2str(size: number) {
    if (size < 1024) {
        return size.toString().concat("b")
    } else if (size < 1024 * 1024) {
        return (size / 1024).toFixed(2) + "kb"
    } else if (size < 1024 * 1024 * 1024) {
        return (size / (1024 * 1024)).toFixed(2) + "mb"
    } else {
        return (size / (1024 * 1024 * 1024)).toFixed(2) + "gb"
    }
}