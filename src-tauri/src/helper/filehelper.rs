use image::GenericImageView;

use crate::server::response::imageres::ImageInfo;
use std::{
    fs::{self, DirBuilder},
    path::Path,
};

/**
 * 确保该路径可用
 * 如果传入的是一个文件夹, 则确保该文件夹已被创建
 * 如果传入的是一个文件, 则确保文件所在的文件夹已被创建
 *
 * 返回是否创建成功
 */
pub fn sure_path<P: AsRef<Path>>(p: P) -> bool {
    let mut tmp = p.as_ref().to_path_buf();
    if tmp.is_file() {
        tmp.pop();
    } else if let Some(_) = tmp.extension() {
        tmp.pop();
    }
    if tmp.is_dir() && !tmp.exists() {
        return DirBuilder::new().recursive(true).create(tmp).is_ok();
    }
    return true;
}

/**
 * 如果传入的是一个文件, 且该文件已存在, 则会删除该文件
 */
pub fn sure_file_new<P: AsRef<Path>>(p: P) -> bool {
    let tmp = p.as_ref();
    if tmp.exists() && tmp.is_file() {
        return fs::remove_file(tmp).is_ok();
    }
    return true;
}

pub fn image_read<P: AsRef<Path>>(p: P) -> anyhow::Result<ImageInfo> {
    let p = p.as_ref();

    if p.exists() {
        if let Ok(i) = image::open(p) {
            if let Some(name) = p.file_name() {
                let name = name.to_os_string().to_str().unwrap_or("").to_string();
                let mut len = 0;
                if let Ok(m) = p.metadata() {
                    len = m.len();
                }
                let path = crate::server::routerfile::FileSaver::path2url(&p);
                let (w, h) = i.dimensions();
                return Ok(ImageInfo::new(name, path, len, w, h));
            }
        }
    }

    return Err(anyhow::anyhow!("error to read image"));
}
