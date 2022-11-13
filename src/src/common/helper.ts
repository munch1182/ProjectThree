

/**
 * 延时处理, 在timeout时间内不再调用才会执行fn
 */
export const delay = (fn: () => void, timeout: number) => {
    let timer: ReturnType<typeof setTimeout>
    return () => {
        timer ? clearTimeout(timer) : null
        timer = setTimeout(fn, timeout);
    }
}