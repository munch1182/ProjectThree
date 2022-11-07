use lazy_static::lazy_static;
use std::sync::Mutex;
pub struct App {}

impl App {
    pub fn set_server_addr(addr: String) {
        if let Ok(mut s_d) = SERVER_ADDR.lock() {
            s_d.addr = Some(addr);
            s_d.start_time = chrono::prelude::Local::now().timestamp_millis();
        }
    }

    pub fn get_server_addr_or_empty() -> String {
        if let Ok(o) = SERVER_ADDR.lock() {
            if let Some(s) = &o.addr {
                return s.clone();
            }
        }
        return "".to_string();
    }

    pub fn get_server_start_time() -> i64 {
        if let Ok(o) = SERVER_ADDR.lock() {
            return o.start_time;
        }
        return 0;
    }
}

struct ServerAddr {
    addr: Option<String>, // 开启的服务器地址
    start_time: i64,      // 服务器创建时间戳
}

impl ServerAddr {
    fn init() -> Self {
        ServerAddr {
            addr: None,
            start_time: 0,
        }
    }
}

lazy_static! {
    static ref SERVER_ADDR: Mutex<ServerAddr> = Mutex::new(ServerAddr::init());
}
