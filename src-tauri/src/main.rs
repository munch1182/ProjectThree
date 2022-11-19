#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod helper;
mod plugin;
mod server;

use crate::plugin::pluginwin::TaruiWindowPlugin;

fn main() {
    helper::init(); // 统一初始化
    std::thread::spawn(|| server::create_server()); // 新线程创建服务器
    create_ui(); // 创建ui
}

fn create_ui() {
    tauri::Builder::default()
        // .setup(|app| setup(app))
        .invoke_handler(tauri::generate_handler![server_or_empty])
        .plugin(TaruiWindowPlugin::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn server_or_empty() -> String {
    app::App::get_server_addr().unwrap_or(String::from(""))
}

// fn setup(app: &mut tauri::App) -> Result<(), Box<dyn Error>> {
//     #[cfg(debug_assertions)]
//     open_dev(app);
//     Ok(())
// }

// fn open_dev(_app: &mut tauri::App) {
//     use log::info;
//     use tauri::Manager;
//     if let Some(w) = _app.get_window("main") {
//         w.open_devtools();
//     } else {
//         info!("error to open devtools");
//     }
// }
