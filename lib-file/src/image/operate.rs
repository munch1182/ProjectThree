use lib::{err, Result};

use crate::{
    file::file::FileInfo,
    image::imgbean::{ImageOperate, ImageOperateExecute},
};

use super::imgbean::ImageRequest;

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
    operate: Vec<(Vec<ImageOperate>, Vec<Option<FileInfo>>)>,
}

impl ImageOperateHelper {
    pub fn from(req: &ImageRequest) -> Result<Self> {
        let file = FileInfo::fromurl(&req.url)?;
        let operate = req.operate.iter().map(|f| (f.clone(), vec![])).collect();
        Ok(Self { file, operate })
    }

    fn _result(&self) -> Vec<Vec<Option<FileInfo>>> {
        let vec = self
            .operate
            .iter()
            .map(|x| x.1.clone())
            .collect::<Vec<Vec<Option<FileInfo>>>>();
        vec
    }

    pub fn operate(&mut self) -> lib::Result<Vec<Vec<Option<FileInfo>>>> {
        let f = &self.file;

        let filepath = f.path();

        let name_no_ext = &filepath.file_stem().ok_or(err!("file no name"))?;

        let newdir = f.dir().join(name_no_ext.clone()); //与文件同名的文件夹
        lib::file::dir_new(&newdir)?; // 清空旧文件

        for (opera, result) in &mut self.operate {
            let mut iter = opera.iter();
            while let Some(o) = iter.next() {
                let dir = &newdir.to_path_buf();
                let src = &filepath.to_path_buf();
                let r = match o {
                    ImageOperate::Convert(p) => p.execute(dir, src),
                    ImageOperate::Resize(p) => p.execute(dir, src),
                    ImageOperate::Flip(p) => p.execute(dir, src),
                    ImageOperate::Blur(p) => p.execute(dir, src),
                    ImageOperate::Rotate(p) => p.execute(dir, src),
                    ImageOperate::Crop(p) => p.execute(dir, src),
                    ImageOperate::Huerotate(p) => p.execute(dir, src),
                };
                lib::debug!("operate: --> {:?}: {}", opera, r.is_some());
                result.push(r);
            }
        }
        Ok(self._result())
    }
}
