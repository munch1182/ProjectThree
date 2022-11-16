use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

lazy_static! {
    static ref SERVER_ADDR: Mutex<ServerAddr> = Mutex::new(ServerAddr::init());
}

pub(crate) struct App {}

impl App {
    pub(crate) fn none() -> ServerAddr {
        ServerAddr {
            addr: None,
            start_time: 0,
        }
    }

    pub(crate) fn set_server(addr: String) {
        if let Ok(mut s_d) = SERVER_ADDR.lock() {
            s_d.addr = Some(addr);
            s_d.start_time = chrono::prelude::Local::now().timestamp_millis();
        }
    }

    pub(crate) fn get_server() -> Option<ServerAddr> {
        if let Ok(o) = SERVER_ADDR.lock() {
            return Some(ServerAddr {
                addr: o.addr.clone(),
                start_time: o.start_time,
            });
        }
        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerAddr {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>, // 开启的服务器地址
    pub start_time: i64, // 服务器创建时间戳
}

impl ServerAddr {
    fn init() -> Self {
        ServerAddr {
            addr: None,
            start_time: 0,
        }
    }
}
