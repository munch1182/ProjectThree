use std::path::Path;

use liblib::{Deserialize_repr, Serialize_repr};
use serde::{Deserialize, Serialize};

use crate::file::FileInfo;

/// 对外可用的图片静态url
pub type ImageAssetUrl = String;

///
/// *外部发起的图片操作请求
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
    /// 指向的图片url
    pub url: ImageAssetUrl,
    /// 要进行的操作, 每外层集合生成一张图片, 内部集合为该图片要操作的流程
    pub operate: Vec<Vec<ImageOperate>>,
}

impl ImageRequest {
    /// 对请求进行执行, 返回的是执行后的文件, 失败则为None
    pub fn operate(&mut self) -> liblib::Result<Vec<Vec<Option<FileInfo>>>> {
        let mut helper = super::operate::ImageOperateHelper::from(self)?;
        helper.operate()
    }
}

/// *图片信息, todo
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageInfo {
    url: ImageAssetUrl, // 对外可用的静态url
    len: u64,           // 文件大小
    dimen: ImageDimen,  // 文件尺寸
    #[serde(skip_serializing_if = "Option::is_none")]
    operate: Option<Vec<ImageOperate>>, // 经过的文件操作, 是原图则为null, 注意: 操作可能是一系列的
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<ImageAssetUrl>, // 如果该值经过了变化, 该值对应原图像的url路径, 原图只能是调用时的目标, 中间变换的忽略
}

/// 描述图片尺寸
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDimen {
    /// 宽
    pub w: u32,
    /// 高
    pub h: u32,
}

///
/// *图片能进行的操作, 注意与rust与json的转换的不同
///
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ImageOperate {
    /// 将当前图片类型转为其它图片类型
    Convert(ImageConvert),
    /// 更改大小
    Resize(ImageResize),
    /// 图片翻转
    Flip(ImageFlipDirect),
    /// 模糊
    Blur(ImageBlur),
    /// 旋转角度, 不能大于360
    Rotate(ImageRotate),
    /// 剪切
    Crop(ImageCrop),
    /// 色彩旋转
    Huerotate(ImageHuerotate),
}

impl ImageOperate {
    pub(super) fn execute<P: AsRef<Path>>(&self, dir: P, src: P) -> Option<FileInfo> {
        match self {
            ImageOperate::Convert(p) => p.execute(dir, src),
            ImageOperate::Resize(p) => p.execute(dir, src),
            ImageOperate::Flip(p) => p.execute(dir, src),
            ImageOperate::Blur(p) => p.execute(dir, src),
            ImageOperate::Rotate(p) => p.execute(dir, src),
            ImageOperate::Crop(p) => p.execute(dir, src),
            ImageOperate::Huerotate(p) => p.execute(dir, src),
        }
    }
}

/// 色彩旋转
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct ImageHuerotate {
    /// 色彩旋转
    pub rotate: i32,
}

/// 图片模糊
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct ImageBlur {
    /// 模糊, blur/100
    pub blur: u8,
}

/// 图片剪切
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct ImageCrop {
    /// x = 0
    pub x: u32,
    /// y = 0
    pub y: u32,
    /// 剪切 x+w
    pub w: u32,
    /// 剪切 y+h
    pub h: u32,
}

/// 图片选中, 只支持固定的角度
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy)]
#[repr(u16)]
pub enum ImageRotate {
    /// 90°
    R90 = 90,
    /// 180°
    R180 = 180,
    /// 270°
    R270 = 270,
}

/// 图片翻转
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum ImageFlipDirect {
    /// 水平方向翻转
    Horizontal = 0,
    /// 垂直方法翻转
    Vertical = 1,
}

/// 更改图片大小
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct ImageResize {
    /// 宽
    pub w: u32,
    /// 高
    pub h: u32,
}

/// 图片类型转换
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum ImageConvert {
    /// ico比较特殊, 前置操作更改了多少size, 都会将其添加进ico中
    Ico = 0,
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
    fn execute<P: AsRef<Path>>(&self, dir: P, src: P) -> Option<FileInfo> {
        let dir = dir.as_ref();
        let src = src.as_ref();
        if let Ok(_f) = self.newname(dir, src) {
            if let Ok(di) = image::open(src) {
                let di = self.execute_by_img(&di);
                if di.save(_f.path()).is_ok() {
                    return Some(_f);
                }
            }
        }
        None
    }
}
impl ImageOperateExecuteByImg for ImageConvert {
    fn execute_by_img(&self, _di: &image::DynamicImage) -> image::DynamicImage {
        match self {
            ImageConvert::Ico => unimplemented!(), // 对于ico, 需要单独处理
        }
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
    fn newname<P: AsRef<Path>>(&self, _dir: P, _src: P) -> liblib::Result<FileInfo> {
        let dir = _dir.as_ref().to_path_buf();
        let src = _src.as_ref();
        use liblib::err;
        let mut f = src.file_stem().ok_or(err!("no name"))?.to_os_string();
        let ext = src.extension().ok_or(err!("no ext"))?;
        f.push(self.name());
        f.push(".");
        f.push(ext);
        Ok(FileInfo::new(dir.join(f)))
    }

    fn name(&self) -> String;
}
