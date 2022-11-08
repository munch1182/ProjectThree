import { WebviewWindow, appWindow } from "@tauri-apps/api/window";
import { invoke } from '@tauri-apps/api'
import log from "./log";

/**
 * 
 * @param router 路由path
 * @returns 该路由path对应的win
 */
function getWinByRouter(router: string): WebviewWindow | null {
    return WebviewWindow.getByLabel(url2Label(router))
}

/**
 * todo 使用net-api来新建页面
 * 
 * @param router vue-router的路由, 用于指定路径
 * @param lebal WebviewWindow的lebel, 用于标记窗口, 新建的窗口默认直接使用router, 新的window默认没有系统标题栏
 * @returns 新建即创建
 */
function newWinow(router: string, lebal: string = router) {
    invoke('create_window', { lebal, router }).then(async () => await currWin().setDecorations(false));
}

/**
 * @param router 路由path
 * @returns win的label
 */
function url2Label(router: string): string {
    if (router == "/") { // 启动页splash在tauri的配置文件中
        return "splash"
    }
    return router
}

/**
 * 当前有焦点的winow
 */
function currWin(): WebviewWindow {
    return appWindow
}

export { currWin, getWinByRouter, newWinow }