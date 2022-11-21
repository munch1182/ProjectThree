use std::path::{Path, PathBuf};

use image::GenericImageView;
use lib::{Deserialize_repr, Serialize_repr};
use serde::{Deserialize, Serialize};

use crate::{file::file::FileInfo, Error};

/// 对外可用的图片静态url
pub type ImageAssetUrl = String;

///
/// 外部发起的图片操作请求
///
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageRequest {
    url: ImageAssetUrl,         // 指向的图片url
    operate: Vec<ImageOperate>, // 要进行的操作
}

type IsTmp = bool;

///
/// 实际操作类
///
#[derive(Debug)]
pub struct ImageOperateHelper {
    start: ImageInfo,             // 最开始操作的文件
    req: Vec<ImageOperate>,       // 要进行的操作
    buf: Vec<(ImageInfo, IsTmp)>, // 中间进行的操作的缓存, 第二个值bool代表该值是否会在结束时删除该文件
}

///
/// 从ImageRequest中构建ImageOperateHelper
///
impl TryFrom<ImageRequest> for ImageOperateHelper {
    type Error = crate::Error;

    fn try_from(value: ImageRequest) -> Result<Self, Self::Error> {
        let file = FileInfo::from(value.url)?;
        let start = file.try_into()?; // file => image
        Ok(Self {
            start,
            req: value.operate,
            buf: vec![],
        })
    }
}

impl ImageOperateHelper {
    pub fn operate(&mut self) -> Result<(), crate::Error> {
        while let Some(i) = self.req.iter().next() {
            (self)._operate(*i, false)?;
        }
        Ok(())
    }

    fn _operate(&mut self, _operate: ImageOperate, istmp: IsTmp) -> Result<(), crate::Error> {
        let curr = if self.buf.is_empty() {
            &self.start // 如果是第一次操作, 那么操作的对象是开始的图片
        } else {
            use lib::err; // 如果不是, 那么操作的对象是最后一次生成的图片
            &lib::option2result!(self.buf.last())?.0
        };
        let info = curr.operate(_operate)?;
        self.buf.push((info, istmp)); // 将这次成功操作的结果保存

        Ok(())
    }
}

///
/// 作为回复的imageinfo(返回时应该是个集合)
///
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageInfo {
    url: ImageAssetUrl, // 对外可用的静态url
    len: u64,           // 文件大小
    dimen: ImageDimen,  // 文件尺寸
    #[serde(skip_serializing_if = "Option::is_none")]
    operate: Option<Vec<ImageOperate>>, // 经过的文件操作, 是原图则为null, 注意: 操作可能是一系列的, 但是ImageOperate::Resize只能是最后一步
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<ImageAssetUrl>, // 如果该值经过了变化, 该值对应原图像的url路径, 原图只能是调用时的目标, 中间变换的忽略
}

///
/// 从FileInfo构建ImageInfo
///
impl TryFrom<FileInfo> for ImageInfo {
    type Error = crate::Error;

    fn try_from(file: FileInfo) -> Result<Self, Self::Error> {
        let url = file.url()?;
        let path = file.path();
        use lib::err;
        let i = lib::err_to!(image::open(&path))?;
        let (w, h) = i.dimensions();
        let dimen = ImageDimen { w, h };
        let len = path.metadata()?.len();
        Ok(Self {
            url,
            len,
            dimen,
            operate: None,
            target: None,
        })
    }
}

impl ImageInfo {
    pub fn from<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = FileInfo::new(path);
        file.try_into()
    }

    ///
    /// 每一次操作都会返回一个新的ImageInfo对象
    ///
    pub fn operate(&self, _operate: ImageOperate) -> Result<Self, Error> {
        let newpath = match _operate {
            ImageOperate::Convert(convert) => self._convert(convert),
            ImageOperate::Resize(resize) => self._resize(resize),
            ImageOperate::Flip(direct) => self._flip(direct),
            ImageOperate::Blur(blur) => self._blur(blur),
            ImageOperate::Rotate(rotate) => self._rotate(rotate),
            ImageOperate::Crop(crop) => self._crop(crop),
        }?;

        // 新的图片
        let mut newimage = Self::from(newpath)?;

        // 更新操作
        newimage.operate = match &self.operate {
            Some(o) => {
                let mut new = o.clone();
                new.push(_operate);
                Some(new)
            }
            None => Some(vec![_operate]),
        };

        // 更新target
        newimage.target = match &self.target {
            Some(t) => Some(t.to_string()),
            None => Some(self.url.clone()),
        };
        Ok(newimage)
    }

    ///
    /// 当转换完成时, 如果该转换是经历了多步, 则应该删除中间步骤
    ///
    fn _delbuf(&self) -> Result<(), Error> {
        Ok(())
    }

    fn _convert(&self, _convert: ImageConvert) -> Result<PathBuf, Error> {
        match _convert {
            ImageConvert::Ico => todo!(),
        }
    }

    fn _resize(&self, _resize: ImageResize) -> Result<PathBuf, Error> {
        todo!()
    }

    fn _flip(&self, _direct: ImageFlipDirect) -> Result<PathBuf, Error> {
        todo!()
    }

    fn _blur(&self, _blur: u32) -> Result<PathBuf, Error> {
        todo!()
    }

    fn _rotate(&self, _rotate: u16) -> Result<PathBuf, Error> {
        todo!()
    }

    fn _crop(&self, _crop: ImageCrop) -> Result<PathBuf, Error> {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDimen {
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ImageOperate {
    Convert(ImageConvert), // 将当前图片类型转为其它图片类型
    Resize(ImageResize),   // 更改大小
    Flip(ImageFlipDirect), // 图片翻转
    Blur(u32),             // 模糊
    Rotate(u16),           // 旋转角度, 不能大于360
    Crop(ImageCrop),       // 剪切
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct ImageCrop {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum ImageRotate {
    R30 = 0,
    R90 = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum ImageFlipDirect {
    Horizontal = 0,
    Vertical = 1,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct ImageResize {
    w: u32,
    h: u32,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
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
        let img = ImageInfo::from("C:\\Users\\yf-2011-24-01\\.p3\\cache\\32x32.png")?;
        println!("{:?}", img);

        let str = lib::to_string(&img)?;
        println!("str: {}", str);

        let ben: ImageInfo = lib::from_str(&str)?;
        println!("info: {:?}", ben);
        Ok(())
    }
}
