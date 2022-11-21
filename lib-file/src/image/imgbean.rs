use std::path::PathBuf;

use lib::{err, err_to, Deserialize_repr, Result, Serialize_repr};
use serde::{Deserialize, Serialize};

use crate::file::file::FileInfo;

/// 对外可用的图片静态url
pub type ImageAssetUrl = String;

///
/// 外部发起的图片操作请求
///
/// 作为回复的imageinfo(返回时应该是个集合)
///
/// ```
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
    url: ImageAssetUrl,              // 指向的图片url
    operate: Vec<Vec<ImageOperate>>, // 要进行的操作, 每外层集合生成一张图片, 内部集合为该图片要操作的流程
}

///
/// 实际处理操作类
///
#[derive(Debug)]
pub struct ImageOperateHelper {
    file: FileInfo, // 指向的图片url
    ///
    /// (Vec<ImageOperate>,Option<FileInfo>):
    /// Vec<ImageOperate>要执行的步骤, 每一步都会保存成一个图片(但是未返回的图片绘制返回后删除)
    /// 可以请求生成执行多个步骤生成多个图片, 即最外层的vec
    operate: Vec<(Vec<ImageOperate>, Vec<FileInfo>)>,
}

// impl Deref for ImageOperateHelper {
//     type Target = Vec<(Vec<ImageOperate>, Vec<FileInfo>)>;

//     fn deref(&self) -> &Self::Target {
//         &self.operate
//     }
// }

// impl DerefMut for ImageOperateHelper {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.operate
//     }
// }

impl ImageOperateHelper {
    pub fn from(req: &ImageRequest) -> Result<Self> {
        let file = FileInfo::fromurl(&req.url)?;
        let operate = req.operate.iter().map(|f| (f.clone(), vec![])).collect();
        Ok(Self { file, operate })
    }

    pub fn result(&self) -> Vec<Vec<FileInfo>> {
        let vec = self
            .operate
            .iter()
            .map(|x| x.1.clone())
            .collect::<Vec<Vec<FileInfo>>>();
        vec
    }

    pub fn operate(&mut self) -> lib::Result<&mut Self> {
        let f = &self.file;

        let filepath = f.path();

        let name_no_ext = &filepath.file_stem().ok_or(err!("file no name"))?;
        let name_had_ext = &filepath.file_name().ok_or(err!(""))?;

        let newdir = f.dir().join(name_no_ext.clone()); //与文件同名的文件夹
        lib::file::dir_new(&newdir)?; // 清空旧文件
        let newfile = newdir.join(name_had_ext.clone()); // 一个新文件夹同名文件地址, 供lib中的方法使用

        for (opera, result) in &mut self.operate {
            let src = Box::new(filepath.to_path_buf());

            lib::debug!("operate: --> {:?}", opera);

            let mut iter = opera.iter();
            while let Some(o) = iter.next() {
                match o {
                    ImageOperate::Convert(_) => todo!(),
                    ImageOperate::Resize(resize) => {
                        let (w, h) = (resize.w, resize.h);

                        // todo 改为错误忽略
                        // 重命名文件
                        let dest =
                            lib::file::file_name_add(&newfile, format!("_{}x{}", w, h).as_str())?;

                        lib::file::file_new(&dest)?;
                        Self::_resize(&dest, src.to_path_buf(), w, h)?;

                        let newf = FileInfo::newfile(&dest)?;

                        lib::debug!("resize: --> {}", dest.display());

                        result.push(newf);
                    }
                    ImageOperate::Flip(_) => todo!(),
                    ImageOperate::Blur(_) => todo!(),
                    ImageOperate::Rotate(_) => todo!(),
                    ImageOperate::Crop(_) => todo!(),
                }
            }
        }
        Ok(self)
    }

    fn _resize(dest: &PathBuf, src: PathBuf, w: u32, h: u32) -> Result<()> {
        let di = err_to!(image::open(src))?;
        let di = di.resize(w, h, image::imageops::FilterType::Nearest);
        err_to!(di.save(dest))?; // 保存到文件
        Ok(())
    }

    ///// 转为ico, dest目标文件路径, src: 源文件路径(ico存放多种大小的图片)
    // fn _convert_icon(dest: PathBuf, src: Vec<PathBuf>) -> Result<()> {
    //     let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    //     let mut ok_count = 0u8;
    //     while let Some(s) = src.iter().next() {
    //         if let Ok(f) = std::fs::File::open(s) {
    //             if let Ok(image) = ico::IconImage::read_png(f) {
    //                 if let Ok(b) = ico::IconDirEntry::encode(&image) {
    //                     icon_dir.add_entry(b);
    //                     ok_count += 1;
    //                 }
    //             }
    //         }
    //     }
    //     if ok_count == 0 {
    //         return Err(err!("error to convert icon"));
    //     }
    //     lib::file::file_new(&dest).ok();
    //     let f = std::fs::File::create(dest)?;
    //     icon_dir.write(f)?;
    //     Ok(())
    // }
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
#[repr(u16)]
pub enum ImageRotate {
    R90 = 90,
    R180 = 180,
    R270 = 270,
    R360 = 360,
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
    use lib::err;

    // cargo.exe test -- image::imgbean::tests::test_img_req --exact --nocapture
    #[test]
    fn test_img_req() -> lib::Result<()> {
        let req = r#"
        {
            "url": "/a/a.png",
            "operate": [
                [
                    {
                        "resize":{
                            "w":16,
                            "h":16
                        }
                    },
                    {
                        "convert":0
                    }
                ],
                [
                    {
                        "flip":0 
                    }, 
                ]
            ]
        }
        "#;
        let req: lib::Result<ImageRequest> = lib::err_to!(lib::from_str(req));
        println!("req: {:?}", req);

        let req2 = ImageRequest {
            url: "/a/a.png".to_string(),
            operate: vec![
                vec![
                    ImageOperate::Resize(ImageResize { w: 16, h: 16 }),
                    ImageOperate::Convert(ImageConvert::Ico),
                ],
                vec![ImageOperate::Flip(ImageFlipDirect::Horizontal)],
            ],
        };
        let req2 = lib::err_to!(lib::to_string(&req2))?;
        println!("req2: {}", req2);
        Ok(())
    }

    // cargo.exe test -- image::imgbean::tests::test_opera --exact --nocapture
    #[test]
    fn test_opera() -> lib::Result<()> {
        let req2 = ImageRequest {
            url: "/a/a.png".to_string(),
            operate: vec![
                vec![ImageOperate::Resize(ImageResize { w: 16, h: 16 })],
                vec![ImageOperate::Resize(ImageResize { w: 32, h: 32 })],
                vec![ImageOperate::Resize(ImageResize { w: 48, h: 48 })],
                vec![ImageOperate::Resize(ImageResize { w: 16, h: 16 })],
            ],
        };

        println!("{:?}", req2);
        let mut helper = ImageOperateHelper::from(&req2)?;
        let result = helper.operate()?.result();

        println!("{:?}", result);
        Ok(())
    }
}
