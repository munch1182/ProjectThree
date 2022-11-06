#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod appresult;
mod log;
mod server;

use ::log::{error, info};
use app::App;

fn main() {
    log::init_log(); // 初始化日志

    std::thread::spawn(|| server::create_server()); // 在另一个线程中创建服务

    create_tauri();
}

fn create_tauri() {
    info!("create tauri.");
    let tauri = tauri::Builder::default() // 创建UI
        .invoke_handler(tauri::generate_handler![server_addr, log_from_js])
        .run(tauri::generate_context!()); // 如果没有dist, 就新建一个dist, 该dist应该指向前端的正式打包文件夹
    if let Err(e) = tauri {
        error!("error tauri: {}", e);
    }
}

#[tauri::command]
async fn server_addr() -> String {
    App::get_server_addr_or_empty()
}

#[tauri::command]
async fn log_from_js(content: String) {
    info!("{}", content)
}
