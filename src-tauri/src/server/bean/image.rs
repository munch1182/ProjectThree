use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ImageOperate {
    Ico = 0,    // 将文件转为ico文件
    Flip = 1,   // 翻转
    Crop = 2,   // 剪切
    Resize = 3, // 更改大小 / 更改后居中
    Blur = 4,   // 模糊
    Rotate = 5, // 旋转
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageOperateReq {
    pub url: String,                    // 目标文件url,
    pub operate: Option<ImageOperate>,  // 变化目标, 如果是变化而来, 则记录了变化的内容
    pub value: Option<i32>,             // 变化值, 如果变化目标需要
    pub dimen: Option<Vec<ImageDimen>>, // 目标大小, 如果只有大小, 则会改变该url指向的文件大小并返回, 如果都有, 则先变化, 再改变变化后的文件大小并返回
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
    use serde_repr::{Deserialize_repr, Serialize_repr};

    #[derive(Debug, Serialize_repr, Deserialize_repr)]
    #[repr(u8)]
    pub enum TestFlipDirect {
        H = 0,
        V = 1,
    }

    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, Serialize)]
    pub enum TestImageOperate {
        Ico,                        // 将文件转为ico文件
        Flip(TestFlipDirect),       // 翻转
        Crop((u32, u32, u32, u32)), // 剪切
        Resize((u32, u32, bool)),   // 更改大小 / 更改后居中
        Blur(u32),                  // 模糊
        Rotate(u16),                // 旋转
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct TestBean {
        name: String,
        io: Option<TestImageOperate>,
    }
    #[test]
    fn test_json_enum() {
        let result = serde_json::to_string(&TestImageOperate::Crop((0, 0, 16, 16))).unwrap();

        println!("{:?}", result);

        let bean = TestBean {
            name: String::from("test"),
            io: Some(TestImageOperate::Resize((48, 48, true))),
        };

        println!("{:?}", serde_json::to_string(&bean));
    }
}
