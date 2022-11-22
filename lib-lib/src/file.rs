//!
//! [dir_check]
//! [dir_new]
//! [file_new]
//! [file_name_suffix]
//! [file_rename]
//! 
use std::path::PathBuf;

use crate::{err, Result};

///
/// 保证该文件路径存在
///
/// 如果传入的是文件夹, 确保该文件夹存在
/// 如果传入的是文件, 确保该文件所在的文件夹存在
///
pub fn dir_check<P: AsRef<std::path::Path>>(dir: P) -> Result<()> {
    let mut dir = dir.as_ref().to_path_buf();
    if dir.extension().is_some() {
        if !dir.pop() {
            return Err(err!("error to pop path"));
        }
    }
    if !dir.exists() {
        std::fs::DirBuilder::new().recursive(true).create(dir)?; // 创建文件夹
    }
    Ok(())
}

///
/// 保证该文件路径存在且里面为空
///
/// 如果传入的是文件夹, 确保该文件夹存在
/// 如果传入的是文件, 确保该文件所在的文件夹存在
///
pub fn dir_new<P: AsRef<std::path::Path>>(dir: P) -> Result<()> {
    let mut dir = dir.as_ref().to_path_buf();
    if dir.extension().is_some() {
        if !dir.pop() {
            return Err(err!("error to pop path"));
        }
    }
    if dir.exists() {
        // 如果已存在则删除文件夹
        std::fs::remove_dir_all(&dir)?;
    }
    if !dir.exists() {
        std::fs::DirBuilder::new().recursive(true).create(dir)?; // 创建文件夹
    }
    Ok(())
}

///
/// 确保该文件是最新的
///
/// 如果传入的是文件夹, 则会确保该文件夹已存在
/// 如果传入的是文件, 会确保该文件夹已存在, 且如果该文件已存在, 则会删除该文件
///  
pub fn file_new<P: AsRef<std::path::Path>>(path: P) -> Result<()> {
    dir_check(path.as_ref())?;
    let path = path.as_ref();
    if path.extension().is_some() {
        if path.exists() {
            std::fs::remove_file(path)?;
        }
    }
    Ok(())
}

///
/// 给文件或者文件夹添加后缀名称(文件的名称默认添加在后缀名之前), 不会判断文件夹是否存在
///
/// add: _16x16
/// 文件夹: \\b\\c => \\b\\c_16x16
/// 文件: a.png => a_16x16.png
///
/// [file_rename]: file_rename
///
pub fn file_name_suffix<P, S>(path: P, add: &S) -> Result<PathBuf>
where
    P: AsRef<std::path::Path>,
    S: AsRef<std::ffi::OsStr> + ?Sized,
{
    file_rename(path, |name| _addname_default(name, add.as_ref()))
}

fn _addname_default(
    oldname: &std::ffi::OsStr,
    addname: &std::ffi::OsStr,
) -> Result<std::ffi::OsString> {
    let oldname = oldname.to_os_string();
    let oldnamestr = oldname.to_str().ok_or(err!())?;

    // 如果带后缀, 则在后缀前添加
    if oldnamestr.contains(".") {
        let addnamestr = addname.to_str().ok_or(err!())?;
        let mut names: Vec<&str> = oldnamestr.split(".").collect();
        let len = names.len();
        if len > 1 {
            names.insert(len - 1, addnamestr);
            names.insert(len, "."); // 因为上面插入了一个但是len值没有变化

            let str: String = names.iter().map(|s| *s).collect();
            return Ok(std::ffi::OsString::from(str));
        }
    }
    let mut result = oldname.to_os_string();
    result.push(addname.to_os_string());
    return Ok(result);
}

///
/// 重命名当前文件或者文件夹
///
/// naname: 传入的是当前文件或者文件夹的全名, 需要返回新的文件或者文件夹名
///
/// newaneme: 直接添加到后面
///
/// ```ignore
/// file_reanme(path,|name| _addname(name, "_suffix.bak"))
///
/// /**
///  * 直接将名称添加在后面
///  */
/// fn _addname(oldname: &std::ffi::OsStr,addname: &std::ffi::OsStr) -> Result<std::ffi::OsString, std::io::Error> {
///     let mut result = oldname.to_os_string();
///     result.push(addname.to_os_string());
///     Ok(result)
/// }
/// ```
///
pub fn file_rename<P, F, S>(path: P, newname: F) -> Result<PathBuf>
where
    P: AsRef<std::path::Path>,
    F: Fn(&std::ffi::OsStr) -> Result<S>,
    S: AsRef<std::ffi::OsStr> + std::convert::AsRef<std::path::Path>,
{
    let mut path = path.as_ref().to_path_buf();

    let filename = path.file_name().ok_or(err!())?;
    let _newname = newname(filename)?;

    path.pop();
    let result = path.join(_newname);

    Ok(result)
}
