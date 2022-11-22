use std::path::Path;

use lib::{Deserialize_repr, Serialize_repr};
use serde::{Deserialize, Serialize};

use crate::file::file::FileInfo;

/// 对外可用的图片静态url
pub type ImageAssetUrl = String;

///
/// 外部发起的图片操作请求
///
/// 作为回复的imageinfo(返回时应该是个集合)
///
/// ```no
///  let req = r#"
///    {
///        "url": "/a/a.png",
///        "operate": [
///            [{"resize":{"w":16,"h":16}},{"convert":0}], // url经过resize和convert生成一张图片
///            [{"flip":0}] // url经过flip生成一张图片
///         ]
///    }"#;
/// let req2 = ImageRequest {
///    url: "/a/a.png".to_string(),
///    operate: vec![
///        vec![
///            ImageOperate::Resize(ImageResize { w: 16, h: 16 }),
///            ImageOperate::Convert(ImageConvert::Ico),
///        ],
///        vec![ImageOperate::Flip(ImageFlipDirect::Horizontal)],
///    ],
/// };
/// ```
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImageRequest {
    pub url: ImageAssetUrl,              // 指向的图片url
    pub operate: Vec<Vec<ImageOperate>>, // 要进行的操作, 每外层集合生成一张图片, 内部集合为该图片要操作的流程
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDimen {
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ImageOperate {
    Convert(ImageConvert),     // 将当前图片类型转为其它图片类型
    Resize(ImageResize),       // 更改大小
    Flip(ImageFlipDirect),     // 图片翻转
    Blur(ImageBlur),           // 模糊
    Rotate(ImageRotate),       // 旋转角度, 不能大于360
    Crop(ImageCrop),           // 剪切
    Huerotate(ImageHuerotate), // 色彩旋转
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct ImageHuerotate {
    pub rotate: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct ImageBlur {
    pub blur: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct ImageCrop {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy)]
#[repr(u16)]
pub enum ImageRotate {
    R90 = 90,
    R180 = 180,
    R270 = 270,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum ImageFlipDirect {
    Horizontal = 0,
    Vertical = 1,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct ImageResize {
    pub w: u32,
    pub h: u32,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum ImageConvert {
    Ico = 0, // ico比较特殊, 前置操作更改了多了size, 就会将这些图片添加进ico中
}

impl ImageOperateExecute for ImageConvert {}
impl ImageOperateExecute for ImageResize {}
impl ImageOperateExecute for ImageFlipDirect {}
impl ImageOperateExecute for ImageCrop {}
impl ImageOperateExecute for ImageBlur {}
impl ImageOperateExecute for ImageRotate {}
impl ImageOperateExecute for ImageHuerotate {}

/// 根据操作进行执行
pub(super) trait ImageOperateExecute:
    ImageOperateNewName + ImageOperateExecuteByImg
{
    ///
    /// dir 要保存到的文件夹
    /// src 要操作的源文件
    ///
    /// 返回结果, 如果失败返回none
    ///
    fn execute<P: AsRef<Path>>(&self, _dir: P, _src: P) -> Option<FileInfo> {
        let dir = _dir.as_ref();
        let src = _src.as_ref();
        if let Ok(_f) = self.newname(dir, src) {
            if let Ok(di) = image::open(_src) {
                self.execute_by_img(&di);
                return Some(_f);
            }
        }
        None
    }
}
impl ImageOperateExecuteByImg for ImageConvert {
    fn execute_by_img(&self, _di: &image::DynamicImage) -> image::DynamicImage {
        todo!()
    }
}
impl ImageOperateExecuteByImg for ImageRotate {
    fn execute_by_img(&self, di: &image::DynamicImage) -> image::DynamicImage {
        match self {
            ImageRotate::R90 => di.rotate90(),
            ImageRotate::R180 => di.rotate180(),
            ImageRotate::R270 => di.rotate270(),
        }
    }
}
impl ImageOperateExecuteByImg for ImageFlipDirect {
    fn execute_by_img(&self, di: &image::DynamicImage) -> image::DynamicImage {
        match self {
            ImageFlipDirect::Horizontal => di.fliph(),
            ImageFlipDirect::Vertical => di.flipv(),
        }
    }
}
impl ImageOperateExecuteByImg for ImageCrop {
    fn execute_by_img(&self, di: &image::DynamicImage) -> image::DynamicImage {
        di.crop_imm(self.x, self.y, self.w, self.h)
    }
}
impl ImageOperateExecuteByImg for ImageBlur {
    fn execute_by_img(&self, di: &image::DynamicImage) -> image::DynamicImage {
        di.blur(self.blur as f32 / 100f32)
    }
}
impl ImageOperateExecuteByImg for ImageResize {
    fn execute_by_img(&self, di: &image::DynamicImage) -> image::DynamicImage {
        di.resize(self.w, self.h, image::imageops::FilterType::Nearest)
    }
}
impl ImageOperateExecuteByImg for ImageHuerotate {
    fn execute_by_img(&self, di: &image::DynamicImage) -> image::DynamicImage {
        di.huerotate(self.rotate)
    }
}

pub(super) trait ImageOperateExecuteByImg {
    fn execute_by_img(&self, di: &image::DynamicImage) -> image::DynamicImage;
}

impl ImageOperateNewName for ImageRotate {
    fn name(&self) -> String {
        format!("_rotate_{:?}", self)
    }
}
impl ImageOperateNewName for ImageCrop {
    fn name(&self) -> String {
        format!("_crop_{}_{}_{}x{}", self.x, self.y, self.w, self.h)
    }
}
impl ImageOperateNewName for ImageBlur {
    fn name(&self) -> String {
        format!("_blur_{}", self.blur)
    }
}
impl ImageOperateNewName for ImageResize {
    fn name(&self) -> String {
        format!("_{}x{}", self.w, self.h)
    }
}
impl ImageOperateNewName for ImageFlipDirect {
    fn name(&self) -> String {
        match self {
            ImageFlipDirect::Horizontal => format!("_fliph"),
            ImageFlipDirect::Vertical => format!("_flipv"),
        }
    }
}
impl ImageOperateNewName for ImageConvert {
    fn name(&self) -> String {
        format!("_to_{:?}", self)
    }
}
impl ImageOperateNewName for ImageHuerotate {
    fn name(&self) -> String {
        format!("_huerotate_{:?}", self.rotate)
    }
}

/// 更加操作新建带有操作的文件名
pub(super) trait ImageOperateNewName {
    ///
    /// dir 要保存到的文件夹
    /// src 要操作的源文件
    ///
    /// 返回带有操作的文件名
    ///
    /// exp:
    ///
    /// a.png => a_16x16.png
    ///
    fn newname<P: AsRef<Path>>(&self, _dir: P, _src: P) -> lib::Result<FileInfo> {
        let dir = _dir.as_ref().to_path_buf();
        let src = _src.as_ref();
        use lib::err;
        let mut f = lib::option2result!(src.file_stem())?.to_os_string();
        let ext = lib::option2result!(src.extension())?;
        f.push(self.name());
        f.push(ext);
        Ok(FileInfo::new(dir.join(f)))
    }

    fn name(&self) -> String;
}
