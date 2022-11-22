use liblib::{err, err_to};

use std::{
    ffi::OsString,
    io::Error,
    ops::Add,
    path::{Path, PathBuf},
};

///
/// 对外提供应用的目录
///
///
#[derive(Debug)]
pub struct DirHelper {
    _cachedir: PathBuf,  // 缓存文件根目录
    _configdir: PathBuf, // 配置文件根目录
    _datadir: PathBuf,   // 数据文件根目录
    _url_prefix: String, // 当路径转成url时, 在前面添加的路径
}

impl DirHelper {
    ///
    /// 初始化, 获取路径
    ///
    /// 在user()文件夹下创建rootdirname文件夹, 提供url_prefix以供文件和url相互转换
    ///
    pub fn init(rootdirname: &'static str, url_prefix: &'static str) -> Result<Self, Error> {
        let user = libsys::user().ok_or(err!("no dir"))?;
        let rootdir = PathBuf::from(user).join(rootdirname);
        let cachedir = rootdir.to_path_buf().join("cache");
        let configdir = rootdir.to_path_buf().join("config");
        let datadir = rootdir.to_path_buf().join("data");
        let url_prefix = if url_prefix.starts_with("/") {
            url_prefix[1..].to_string() // 如果以/开头去掉开头的/
        } else {
            url_prefix.to_string()
        };
        return Ok(Self {
            _url_prefix: url_prefix,
            _cachedir: cachedir,
            _configdir: configdir,
            _datadir: datadir,
        });
    }

    pub fn dircache(&self) -> PathBuf {
        self._cachedir.to_path_buf()
    }

    #[allow(dead_code)]
    pub fn dirconfig(&self) -> PathBuf {
        self._configdir.to_path_buf()
    }

    #[allow(dead_code)]
    pub fn dirdata(&self) -> PathBuf {
        self._datadir.to_path_buf()
    }

    ///
    /// 将本地文件路径(只有cache)转为可对外使用的静态文件url(不带baseurl)
    /// 该方法不会检查文件是否存在
    ///
    /// C:\Users\user\.p3\cache\img\a.png => url: /a/img/a.png /a即对外静态文件url
    ///
    pub fn path2url<P: AsRef<Path>>(&self, path: P) -> Result<String, Error> {
        let path = path.as_ref().to_path_buf();

        let cache: &Path = err_to!(path.strip_prefix(&self._cachedir), "error when path2url")?; // 对比path去掉cachedir的部分

        let mut vec: Vec<OsString> = cache.iter().map(|x| x.to_os_string()).collect();
        vec.insert(0, OsString::from(&self._url_prefix)); // 在头部添加通用url路径

        // 此方法正式版暂不能使用
        // let with = vec.iter().intersperse(OsString::from("/"));
        let with: String = vec
            .iter() // 转成iter后自动去掉了路径分隔符
            .map(|x| String::from("/").add(x.to_str().unwrap_or(""))) // 添加url路径分隔符: /
            .collect();
        Ok(with)
    }

    ///
    /// 将静态文件的url转为本地文件路径(只有cache作为基础dir)
    /// 该方法不会检查文件是否存在
    ///
    ///  url: /a/img/a.png => path: C:\Users\user\.p3\cache\img\a.png /a即对外静态文件url
    ///
    pub fn url2path(&self, url: &String) -> Result<PathBuf, Error> {
        let pb = url.split("/").collect::<PathBuf>(); // 将url分割并组成成路径
        let path: &Path = err_to!(pb.strip_prefix(&self._url_prefix))?; // 对比去掉开头部分

        Ok(self.dircache().join(path)) // 组成全路径
    }
}
