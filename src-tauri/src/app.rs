use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Mutex};
use tauri::api::path::data_dir;

lazy_static! {
    static ref SERVER_ADDR: Mutex<ServerAddr> = Mutex::new(ServerAddr::init());
    static ref ASSETS: Mutex<Assets> = Mutex::new(Assets::default());
}

pub(crate) struct App {}

impl App {
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

    pub(crate) fn get_server_addr() -> Option<String> {
        return Self::get_server()?.addr;
    }

    pub fn cachedir() -> PathBuf {
        if let Ok(a) = ASSETS.lock() {
            return a.cache();
        }
        return PathBuf::new();
    }

    pub fn datadir() -> PathBuf {
        if let Ok(a) = ASSETS.lock() {
            return a.data();
        }
        return PathBuf::new();
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

/**
 * 服务器静态文件
 */
struct Assets {
    _cache: PathBuf,
    _data: PathBuf,
}

impl Assets {
    fn default() -> Self {
        let mut dir = data_dir().unwrap_or(PathBuf::from(""));
        dir.push(".p3");
        let data = dir.clone();
        dir.push("cache");
        let cache = dir.clone();
        Self {
            _cache: cache,
            _data: data,
        }
    }

    /**
     * 返回一个缓存文件夹的新路径对象
     */
    fn cache(&self) -> PathBuf {
        self._cache.clone()
    }

    /**
     * 返回一个数据文件夹的新路径对象
     */
    fn data(&self) -> PathBuf {
        self._data.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_assets() {
        let d = super::Assets::default();

        println!("{},{}", d._data.display(), d._cache.display());
    }

    #[test]
    fn test_path() {
        let p_2 = PathBuf::from("aa\\a.txt");

        let p = super::App::cachedir().join(&p_2);

        println!("{},{}", p.display(), p_2.display());

        let p_2 = PathBuf::from("b.txt");

        let p = super::App::cachedir().join(&p_2);

        println!("{},{}", p.display(), p_2.display());
    }
}
