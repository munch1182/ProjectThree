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
    path: PathBuf,
}

impl FileInfo {
    ///
    /// 创建FileInfo对象, 不能保证文件已创建
    ///
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        return Self { path };
    }

    /**
     * 从url中创建FileInfo对象
     */
    pub fn from(url: String) -> Result<Self, Error> {
        let path = err_to!(DIR.lock())?.url2path(&url)?;
        Ok(Self::new(path))
    }

    ///
    /// 获取该对象生成的url
    ///
    pub fn url(&self) -> Result<String, Error> {
        let url = err_to!(DIR.lock())?.path2url(self.path.to_path_buf())?;
        Ok(url)
    }

    ///
    /// 直接获取一个新建的path对象
    ///
    pub fn path(&self) -> PathBuf {
        self.path.to_path_buf()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo.exe test -- file::file::tests::test_file_info --exact --nocapture
    #[test]
    fn test_file_info() {
        let img = FileInfo::new("C:\\Users\\munch\\Downloads\\moon_new.png");

        println!("{:?}", img.url());
    }
}
