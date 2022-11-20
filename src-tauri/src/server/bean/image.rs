use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ImageOperate {
    Ico(u32),              // 将文件转为ico文件, 值为长和宽, 即尺寸
    Flip(ImageFlipDirect), // 翻转
    Crop(ImageCrop),       // 剪切
    Resize(ImageResize),   // 更改大小 / 更改后居中
    Blur(u32),             // 模糊
    Rotate(u16),           // 旋转角度, 不能大于360
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum ImageFlipDirect {
    Horizontal = 0,
    Vertical = 1,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageCrop {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageResize {
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageOperateReq {
    pub url: String,           // 目标文件url,
    pub operate: ImageOperate, // 变化目标
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageInfo {
    pub name: String,                  // 文件名
    pub url: String,                   // 文件url
    pub len: u64,                      // 文件大小
    pub dimen: ImageDimen,             // 文件尺寸
    pub operate: Option<ImageOperate>, // 经过的文件操作, 是原图则为null
    pub target: Option<String>,        // 如果该值经过了变化, 该值对应原图像的url路径
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

    /**
     * 如果此image是通过info改变而来, 使用此方法更新信息
     */
    pub fn from(&mut self, info: &ImageInfo, operate: ImageOperate) {
        self.target = Some(info.url.clone());
        self.operate = Some(operate)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDimen {
    pub w: u32,
    pub h: u32,
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_op() {
        let resize = ImageOperate::Resize(ImageResize { w: 16, h: 16 });
        let op = ImageOperateReq {
            url: "/a/a.test".to_string(),
            operate: resize,
        };

        println!("{:?}", serde_json::to_string(&op).unwrap());

        let str = r#"{"url":"/a/a.test","operate":{"resize":{"w":16,"h":16}}}"#;

        let j: ImageOperateReq = serde_json::from_str(str).unwrap();
        println!("{:?}", j);

        assert_eq!(op, j);
    }

    #[test]
    fn test_op_no_name() {
        let op = ImageOperateReq {
            url: "/a/a.test".to_string(),
            operate: ImageOperate::Ico(128),
        };

        println!("{:?}", serde_json::to_string(&op).unwrap());

        let str = r#"{"url":"/a/a.test","operate":{"ico": 128}}"#;

        let j: ImageOperateReq = serde_json::from_str(str).unwrap();
        println!("{:?}", j);

        assert_eq!(op, j);
    }
}
