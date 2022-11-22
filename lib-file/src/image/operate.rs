use liblib::{err, Result};

use crate::{file::FileInfo, image::imgbean::ImageOperate};

use super::imgbean::ImageRequest;

///
/// 实际处理操作类
///
#[derive(Debug)]
pub(crate) struct ImageOperateHelper {
    file: FileInfo, // 指向的图片url
    ///
    /// (Vec<ImageOperate>,Option<FileInfo>):
    /// Vec<ImageOperate>要执行的步骤, 每一步都会保存成一个图片(但是未返回的图片绘制返回后删除)
    /// 可以请求生成执行多个步骤生成多个图片, 即最外层的vec
    operate: Vec<(Vec<ImageOperate>, Vec<Option<FileInfo>>)>,
}

impl ImageOperateHelper {
    pub(crate) fn from(req: &ImageRequest) -> Result<Self> {
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

    pub(crate) fn operate(&mut self) -> liblib::Result<Vec<Vec<Option<FileInfo>>>> {
        let f = &self.file;

        let filepath = f.path();

        let name_no_ext = &filepath.file_stem().ok_or(err!("file no name"))?;

        let newdir = f.dir().join(name_no_ext.clone()); //与文件同名的文件夹
        liblib::file::dir_new(&newdir)?; // 清空旧文件

        for (opera, result) in &mut self.operate {
            let mut iter = opera.iter();
            while let Some(operate) = iter.next() {
                let dir = &newdir.to_path_buf();
                let src = &filepath.to_path_buf();
                let r = match operate {
                    ImageOperate::Convert(c) => match c {
                        crate::image::imgbean::ImageConvert::Ico => {
                            Self::_convert_ico(result, dir, src).ok()
                        }
                        #[allow(unreachable_patterns)]
                        _ => operate.execute(dir, src),
                    },
                    _ => operate.execute(dir, src),
                };
                result.push(r);
            }
            liblib::debug!("operate: --> {:?}: {}", opera, result.last().is_some());
            println!("operate: --> {:?}: {}", opera, result.last().is_some());
        }
        Ok(self._result())
    }

    fn _convert_ico<P: AsRef<std::path::Path>>(
        result: &mut Vec<Option<FileInfo>>,
        dir: P,
        src: P,
    ) -> Result<FileInfo> {
        let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
        let src = src.as_ref();
        let mut iter = result.iter();
        while let Some(opt) = iter.next() {
            if let Some(f) = opt {
                let file = std::fs::File::open(f.path())?;
                let image = ico::IconImage::read_png(file)?;
                icon_dir.add_entry(ico::IconDirEntry::encode(&image)?);
            }
        }
        let file = std::fs::File::open(src)?;
        let image = ico::IconImage::read_png(file)?;
        icon_dir.add_entry(ico::IconDirEntry::encode(&image)?);

        let mut name = src.file_stem().ok_or(err!())?.to_os_string();
        name.push(".ico");
        let path = dir.as_ref().join(name);
        let f = std::fs::File::create(&path)?;
        icon_dir.write(f)?;
        Ok(FileInfo::new(path))
    }
}
