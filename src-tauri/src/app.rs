use lazy_static::lazy_static;
use std::sync::Mutex;
pub struct App {}

impl App {
    pub fn set_server_addr(addr: String) {
        SERVER_ADDR.lock().unwrap().addr = Some(addr)
    }

    pub fn get_server_addr_or_empty() -> String {
        if let Ok(o) = SERVER_ADDR.lock() {
            if let Some(s) = &o.addr {
                return s.clone();
            }
        }
        return "".to_string();
    }
}

struct ServerAddr {
    addr: Option<String>,
}

impl ServerAddr {
    fn init() -> Self {
        ServerAddr { addr: None }
    }
}

lazy_static! {
    // 开启的服务器地址
    static ref SERVER_ADDR: Mutex<ServerAddr> = Mutex::new(ServerAddr::init());
}
