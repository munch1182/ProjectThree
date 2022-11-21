use lib::{err, err_to};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};
use sys::DirHelper;

use crate::Error;

lib::lazy_static! {
    static ref DIR:Mutex<DirHelper> = Mutex::new(DirHelper::init(".p3", "/a").unwrap()); // 无法处理unwrap
}

#[derive(Debug)]
pub struct FileInfo {
    path: PathBuf,       // 本地文件路径
    url: Option<String>, // 对外保留的url
}

/// 当上传一个文件时, 新建一个FileInfo
///
/// exp:
/// ```
/// let f = FileInfo::newfile("a.png");
/// fs::write(f.path())
/// return f.url()
/// ```
///
///
impl FileInfo {
    pub fn newfile<P: AsRef<Path>>(p: P) -> lib::Result<Self> {
        let path = p.as_ref();
        let path = err_to!(DIR.lock())?.dircache().join(path);
        Ok(Self { path, url: None })
    }

    /// 获取该对象生成的url
    pub fn url(&mut self) -> Result<String, Error> {
        if let Some(url) = &self.url {
            return Ok(url.clone());
        }
        let url = err_to!(DIR.lock())?.path2url(&self.path)?;
        self.url = Some(url.clone());
        Ok(url)
    }

    pub fn path(&self) -> PathBuf {
        self.path.to_path_buf()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo.exe test -- file::file::tests::test_file_info --exact --nocapture
    #[test]
    fn test_file_info() -> lib::Result<()> {
        let name = "a.txt";
        let mut img = FileInfo::newfile(name)?;
        std::fs::write(&img.path, "test fileinfo")?;
        println!(
            "name: {} => path: {} => url: {:?}",
            name,
            &img.path().display(),
            &img.url()?
        );
        Ok(())
    }
}
