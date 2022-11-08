#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod log;
mod server;

use ::log::{error, info};
use app::App;
use tauri::{AppHandle, Manager, Window, WindowBuilder};

fn main() {
    log::init_log(); // 初始化日志

    std::thread::spawn(|| server::create_server()); // 在另一个线程中创建服务

    create_tauri(); // 在当前线程创建tauri
}

fn create_tauri() {
    let tauri = tauri::Builder::default() // 创建UI
        .setup(|app| {
            if let Some(main) = app.get_window("/main") {
                shadow_winow(main);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            server_addr,
            log_from_js,
            create_window
        ]) //只提供这些指令, 其它的功能由服务提供
        .run(tauri::generate_context!()); // 如果没有dist, 就新建一个dist, 该dist应该指向前端的正式打包文件夹
    if let Err(e) = tauri {
        error!("error tauri: {}", e);
    }
}

/**
 * 创建window并附上阴影, ui仍指向router
 */
#[tauri::command]
async fn create_window(handle: AppHandle, lebal: String, router: String) -> bool {
    info!("create_window:{},{}", lebal, router);
    if let Ok(url) = router.parse() {
        let build = WindowBuilder::new(&handle, lebal, tauri::WindowUrl::App(url)) // app本地, url要使用External
            .decorations(false)
            .build();
        if let Ok(win) = build {
            shadow_winow(win);
            return true;
        }
    }
    false
}

fn shadow_winow(win: Window) {
    if let Err(e) = window_shadows::set_shadow(&win, true) {
        info!("error to set window shadeow: {}", e);
    }
}

/**
 * 返回当前服务器地址, 若创建失败, 则返回空字符
 */
#[tauri::command]
async fn server_addr() -> String {
    App::get_server_addr_or_empty()
}

/**
 * 将js中的日志输出到rust中
 */
#[tauri::command]
async fn log_from_js(content: String) {
    info!("{}", content)
}
