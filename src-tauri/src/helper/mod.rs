pub mod filehelper;
mod log;

pub(crate) fn init() {
    log::init_log(); // 日志功能初始化
}
