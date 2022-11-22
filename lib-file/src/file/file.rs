use liblib::{err, err_to, Result};
use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};

use crate::file::path::DirHelper;

liblib::lazy_static! {
    static ref DIR:Mutex<DirHelper> = Mutex::new(DirHelper::init(".p3", "/a").unwrap()); // 无法处理unwrap
}

///
/// 关联文件和url
///
/// url在必要时才会生成, path会一直有值, 但该文件未被真实被创建
///
#[derive(Debug, Clone)]
pub struct FileInfo {
    path: PathBuf,       // 本地文件路径
    url: Option<String>, // 对外保留的url
}

/// 当上传一个文件时, 新建一个FileInfo
///
/// exp:
/// ``` no
/// let f = FileInfo::newfile("a.png");
/// fs::write(f.path())?:
/// return f.url()
/// ```
///
impl FileInfo {
    pub fn new<P: AsRef<Path>>(p: P) -> Self {
        Self {
            path: p.as_ref().to_path_buf(),
            url: None,
        }
    }

    ///
    /// 新建一个file
    ///
    pub fn newfile<P: AsRef<Path>>(p: P) -> Result<Self> {
        let path = p.as_ref();
        let path = err_to!(DIR.lock())?.dircache().join(path);
        Ok(Self::new(path))
    }

    ///
    /// 从url中获取file
    ///
    pub fn fromurl(url: &String) -> Result<Self> {
        let url = url.clone();
        let path = err_to!(DIR.lock())?.url2path(&url)?;
        Ok(Self {
            path,
            url: Some(url),
        })
    }

    /// 获取该对象生成的url
    pub fn url(&mut self) -> Result<String> {
        if let Some(url) = &self.url {
            return Ok(url.clone());
        }
        let url = err_to!(DIR.lock())?.path2url(&self.path)?;
        self.url = Some(url.clone());
        Ok(url)
    }

    /// 返回该文件所在的文件夹
    pub fn dir(&self) -> PathBuf {
        let mut p = self.path();
        if p.is_dir() {
            return p;
        }
        p.pop();
        return p;
    }

    pub fn path(&self) -> PathBuf {
        self.path.to_path_buf()
    }
}
