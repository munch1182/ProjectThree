import { WebviewWindow, appWindow } from "@tauri-apps/api/window";

/**
 * 
 * @param router 路由path
 * @returns 该路由path对应的win
 */
function getWinByRouter(router: string): WebviewWindow | null {
    return WebviewWindow.getByLabel(url2Label(router))
}


function newWinow(router: string, lebal: string = router) {
    new WebviewWindow(lebal, {
        url: router,
        decorations: false
    })
}

/**
 * @param router 路由path
 * @returns win的label
 */
function url2Label(router: string): string {
    return router
}

/**
 * 当前有焦点的winow
 */
function currWin(): WebviewWindow {
    return appWindow
}

function minimizeWindow() {
    const curr = currWin();
    if (!curr) return
    curr.minimize()
}
function closeWindow() {
    const curr = currWin();
    if (!curr) return
    curr.close()
}

async function alawysTop(enable: boolean) {
    await currWin().setAlwaysOnTop(enable)
}

export { closeWindow, minimizeWindow, newWinow, alawysTop }