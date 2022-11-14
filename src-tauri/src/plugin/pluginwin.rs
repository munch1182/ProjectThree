use log::info;
use tauri::{plugin::Plugin, Runtime, Window};

pub struct TaruiWindowPlugin {}

impl TaruiWindowPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl<R: Runtime> Plugin<R> for TaruiWindowPlugin {
    fn name(&self) -> &'static str {
        "tauri-plugin-window"
    }

    fn created(&mut self, window: Window<R>) {
        // 为窗口加上阴影
        if let Err(_) = window_shadows::set_shadow(window, true) {
            info!("error to shadow window.")
        }
    }
}
