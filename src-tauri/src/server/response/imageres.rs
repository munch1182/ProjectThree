use serde::{Deserialize, Serialize};
// use serde_repr::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum ImageOperate {
    // Re,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageInfo {
    name: String,                  // 文件名
    url: String,                   // 文件url
    len: u64,                      // 文件大小
    dimen: ImageDimen,             // 文件尺寸
    operate: Option<ImageOperate>, // 经过的文件操作, 是原图则为null
    target: Option<String>,        // 如果该值经过了变化, 该值对应原图像的url路径
}

impl ImageInfo {
    pub fn new(name: String, url: String, len: u64, w: u32, h: u32) -> Self {
        Self {
            name,
            url,
            len,
            dimen: ImageDimen { w, h },
            operate: None,
            target: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDimen {
    w: u32,
    h: u32,
}
