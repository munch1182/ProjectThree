use std::path::Path;

use image::GenericImageView;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::file::file::FileInfo;

/// 对外可用的图片静态url
pub type ImageAssetUrl = String;

///
///
///
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageInfo {
    url: ImageAssetUrl,                 // 对外可用的静态url
    len: u64,                           // 文件大小
    dimen: ImageDimen,                  // 文件尺寸
    operate: Option<Vec<ImageOperate>>, // 经过的文件操作, 是原图则为null, 注意: 操作可能是一系列的, 但是ImageOperate::Resize只能是最后一步
    target: Option<ImageAssetUrl>, // 如果该值经过了变化, 该值对应原图像的url路径, 原图只能是调用时的目标, 中间变换的忽略
}

impl ImageInfo {
    pub fn from<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let path = path.as_ref();
        let file = FileInfo::new(path);

        use lib::err;
        let i = lib::err_to!(image::open(path))?;
        let (w, h) = i.dimensions();
        let dimen = ImageDimen { w, h };
        let url = file.url()?;
        let path = path.to_path_buf();
        let len = path.metadata()?.len();
        Ok(Self {
            url,
            len,
            dimen,
            operate: None,
            target: None,
        })
    }

    ///
    /// 每一次操作都会返回一个新的ImageInfo对象/ 只保留第一个和最后一个对象?
    ///
    pub fn operate(&mut self, _operate: ImageOperate) -> Self {
        // todo!("需要更改operate生成新的文件名并返回新的文件地址以及target, 以及更新大小")

        if self.target.is_none() {
            self.target = Some(self.url.clone()) // 更新target
        }

        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDimen {
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ImageOperate {
    Convert(ImageConvert), // 将当前图片类型转为其它图片类型
    Resize(ImageResize),   // 更改大小
    Flip(ImageFlipDirect), // 图片翻转
    Blur(u32),             // 模糊
    Rotate(u16),           // 旋转角度, 不能大于360
    Crop(ImageCrop),       // 剪切
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImageCrop {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum ImageRotate {
    R30 = 0,
    R90 = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum ImageFlipDirect {
    Horizontal = 0,
    Vertical = 1,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageResize {
    w: u32,
    h: u32,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum ImageConvert {
    Ico = 0,
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo.exe test -- image::imgbean::tests::test_image --exact --nocapture
    // 测试时该文件须在静态目录下, 否则无法转换url
    #[test]
    fn test_image() -> Result<(), std::io::Error> {
        let img = ImageInfo::from("");
        println!("{:?}", img);
        Ok(())
    }
}
