#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod plugin;
use crate::plugin::pluginwin::TaruiWindowPlugin;

fn main() {
    tauri::Builder::default()
        //.invoke_handler(tauri::generate_handler![greet])
        .plugin(TaruiWindowPlugin::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
