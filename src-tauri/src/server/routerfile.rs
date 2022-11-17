use std::path::PathBuf;
use tauri::api::path::data_dir;

use super::response::baseres::NetBB as BB;
use axum::{extract::Multipart, routing::post, Json, Router};
use axum_extra::routing::SpaRouter;
use log::{error, info};

/**
 * 路由, /i+..
 */
pub(crate) fn router2file() -> Router {
    Router::new().route("/u", post(receive_upload)) // upload
}

/**
 * 静态资源路由, 直接通过路由访问
 */
pub(crate) fn router2assets() -> SpaRouter {
    SpaRouter::new("/a", ServerFile::assets_dir()) // /a/*.*
}

const SUFFIX_FILE: &'static str = "f_";

// todo 支持分段同时发送和接收
async fn receive_upload(mut part: Multipart) -> Json<BB<Vec<String>>> {
    let mut vec = vec![];
    while let Some(f) = part.next_field().await.unwrap_or(None) {
        let content_type = f.content_type().unwrap_or("");

        if content_type.is_empty() {
            // 必须要有类型否则直接失败
            return BB::file_req_err().to();
        }
        let want_name = f.name().unwrap_or(""); // 必须要有format-data前的名字
        if !want_name.starts_with(SUFFIX_FILE) {
            // 且必须以f-开头否则直接失败
            return BB::file_req_err().to();
        }
        let save_path_name = format!(
            "{}_{}",
            want_name.replace(SUFFIX_FILE, ""),
            content_type.replace("/", ".")
        );

        println!("save_path_name: {}", save_path_name);

        if let Ok(data) = f.bytes().await {
            let result = ServerFile::get().cache(&save_path_name, data).await;
            if let Some(name) = result {
                info!("{}", name.display());
                vec.push(format!("/a/{}", save_path_name)); // todo 建表: 返回文件名与实际地址的引用
            } else {
                vec.push(String::new())
            }
        } else {
            error!("error to read file data");
            return BB::file_req_err().to();
        }
    }

    BB::success(vec).to()
}

struct ServerFile {
    _dirdir: PathBuf,   // 数据文件夹
    _cachedir: PathBuf, // 缓存文件夹部分
}

impl ServerFile {
    fn get() -> Self {
        let dir = data_dir().unwrap_or(PathBuf::from("../"));
        let mut _dirdir = PathBuf::new(); // {data}/.p3  => {user}/AppData/Roaming/.p3
        _dirdir.clone_from(&dir);
        _dirdir.push(".p3");
        let mut _cachedir = PathBuf::new(); // {data}/.p3/cache
        _cachedir.clone_from(&_dirdir);
        _cachedir.push("cache");
        Self { _dirdir, _cachedir }
    }

    fn assets_dir() -> PathBuf {
        let dir = data_dir().unwrap_or(PathBuf::from("../"));
        let mut _cachedir = PathBuf::new(); // {data}/.p3/cache
        _cachedir.clone_from(&dir);
        _cachedir.push(".p3");
        _cachedir.push("cache");
        _cachedir
    }

    /**
     * 将数据写入缓存文件夹下的[filename]路径中, 返回最后的文件名
     */
    async fn cache(&self, filename: &String, data: impl AsRef<[u8]>) -> Option<PathBuf> {
        Self::_write(&self._cachedir, filename, data).await
    }

    /**
     * 将数据写入数据文件夹下的[filename]路径中, 返回最后的文件名
     */
    // async fn data(&self, filename: &String, data: impl AsRef<[u8]>) -> Option<PathBuf> {
    //     Self::_write(&self._datadir, filename, data).await
    // }

    async fn _write(dir: &PathBuf, filename: &String, data: impl AsRef<[u8]>) -> Option<PathBuf> {
        if !Self::_mk_assets_dir(dir) {
            return None;
        }
        let mut filepath = PathBuf::new();
        filepath.push(dir.as_os_str());
        filepath.push(filename);

        // 如果有同名文件, 则删除原文件
        if filepath.exists() {
            if let Err(e) = std::fs::remove_file(&filepath) {
                error!("error to del {}: {}", &filepath.display(), e);
                return None;
            }
        }

        if let Ok(_) = tokio::fs::write(&filepath, data).await {
            return Some(filepath);
        } else {
            error!("error to write data to file: {}", filepath.display());
        }
        return None;
    }

    fn _mk_assets_dir(dir: &PathBuf) -> bool {
        use std::fs;
        if let Err(_) = fs::read_dir(dir) {
            if let Err(e) = fs::DirBuilder::new().recursive(true).create(dir) {
                error!("error to create dir: {}", e);
                return false;
            } else {
                info!("success to create dir: {}", dir.display());
            }
        }
        return true;
    }
}
