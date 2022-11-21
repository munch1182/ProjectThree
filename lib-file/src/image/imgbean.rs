use lib::{Deserialize_repr, Serialize_repr};
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Deserialize, Serialize)]
pub struct ImageRequest {
    url: ImageAssetUrl,              // 指向的图片url
    operate: Vec<Vec<ImageOperate>>, // 要进行的操作, 每外层集合生成一张图片, 内部集合为该图片要操作的流程
}

///
/// 实际处理操作类
///
#[derive(Debug)]
pub struct ImageOperateHelper {
    _req: ImageRequest,   // 请求
    _tmp: Vec<ImageInfo>, // 处理过程中产生的所有图片
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
}
