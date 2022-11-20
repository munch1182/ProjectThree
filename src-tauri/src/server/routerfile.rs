use std::ffi::OsString;
use std::path::{Path, PathBuf};

use super::bean::baseres::NetBB as BB;
use super::bean::image::{ImageInfo, ImageOperateReq};
use crate::app::App;
use axum::body::Bytes;
use axum::Json;
use axum::{extract::Multipart, routing::post, Router};
use axum_extra::routing::SpaRouter;
use log::{debug, error};

/**
 * 路由, /i+..
 */
pub(crate) fn router2file() -> Router {
    Router::new()
        .route("/u", post(receive_upload)) // upload
        .route("/i/u", post(img_receive_upload)) // 图片上传, 图片处理是单独的路由
        .route("/i/1", post(img_operate)) // 图片变化, 图片处理是单独的路由
}

/**
 * 静态资源路由, 直接通过路由访问
 */
pub(crate) fn router2assets() -> SpaRouter {
    SpaRouter::new(SUFFIX_ROUTER, App::cachedir()) // /a/*.*
}

async fn receive_upload(mut part: Multipart) -> Json<BB<Vec<String>>> {
    let mut vec = vec![];
    while let Ok(Some(field)) = part.next_field().await {
        if let Some(tname) = field.name() {
            // 对应Content-Type后的文件名字
            if !tname.starts_with(SUFFIX_FILE) {
                break; // name没有前缀则不保存, 直接结束
            }
            if let Some(fname) = field.file_name() {
                if fname.is_empty() {
                    break; // 无文件名则不保存, 直接结束
                }
                let reqname = format!("{}_{}", tname, fname);
                let path = Path::new(reqname.as_str());
                if let Ok(b) = field.bytes().await {
                    // todo 判断保存的文件夹和路径(根据tname)
                    if let Ok(name) = FileSaver::cache(path, b).await {
                        vec.push(name);
                    }
                }
            }
        }
    }
    BB::success(vec).to()
}

async fn img_operate(
    payload: Option<Json<Vec<ImageOperateReq>>>,
) -> Json<BB<Vec<Option<ImageInfo>>>> {
    if let Some(Json(req)) = payload {
        let mut vec = vec![];
        for ele in req {
            let f = FileSaver::url2path(&App::cachedir(), &ele.url);

            if !f.exists() || !f.is_file() {
                // 该图片不存在
                return BB::file_req_err().to();
            }

            use crate::helper::filehelper::imgae_operate;
            if let Ok(info) = imgae_operate(f, &ele.operate) {
                vec.push(Some(info));
            } else {
                vec.push(None);
            }
        }
        return BB::success(vec).to();
    }

    BB::fail().to()
}

async fn img_receive_upload(part: Multipart) -> Json<BB<Vec<Option<ImageInfo>>>> {
    let result = receive_upload(part).await.0; // 先将图片保存
    if result.issuccess() {
        if let Some(ref vec) = result.data {
            let dir = &App::cachedir();
            let mut resultvec = vec![];
            for ele in vec {
                let path = FileSaver::url2path(dir, &ele);

                if let Ok(info) = crate::helper::filehelper::image_read(&path) {
                    resultvec.push(Some(info));
                } else {
                    resultvec.push(None);
                }
            }
            return BB::success(resultvec).to();
        }
    }
    return BB::none_from(&result).to();
}

const SUFFIX_FILE: &'static str = "f_";
const SUFFIX_ROUTER: &'static str = "/a";

pub struct FileSaver {}

impl FileSaver {
    /**
     * 将返回的静态文件路径转为在dir路径下的真实文件路径
     */
    pub fn url2path(dir: &PathBuf, url: &String) -> PathBuf {
        let urlpath = PathBuf::from(url); // 以SUFFIX_ROUTER开头的地址

        // 将SUFFIX_ROUTER去掉
        let filepath = urlpath
            .iter()
            .map(|x| x.to_os_string())
            .collect::<Vec<OsString>>()[2..] // 因为是/a => / + a, 所以是从2开始
            .iter()
            .collect::<PathBuf>();
        return dir.join(filepath);
    }

    /**
     * 将路径转为对外可访问的url
     */
    pub fn path2url<P: AsRef<Path>>(p: P) -> String {
        let mut path = p.as_ref().to_path_buf();

        let cache = App::cachedir();
        let ds = cache.as_os_str();
        let mut vec = vec![];
        loop {
            let pathstr = path.as_os_str();

            if pathstr == ds {
                vec.reverse(); // 需要倒转
                let finalpath: PathBuf = vec.iter().collect();
                return format!("{}/{}", SUFFIX_ROUTER, finalpath.to_str().unwrap_or(""))
                    .replace("\\", "/");
            }
            if let Some(filename) = path.file_name() {
                vec.push(filename.to_os_string());
            }
            if !path.pop() {
                break;
            };
        }

        String::new()
    }

    /**
     * 将文件写入缓存文件
     *
     * path: 文件名, 可带文件夹, 但不能以/开头
     * 返回最终的文件服务器地址(不带baseurl), 如果写入失败, 则返回空字符
     */
    async fn cache<P>(path: P, data: Bytes) -> anyhow::Result<String>
    where
        P: AsRef<Path> + std::fmt::Debug,
    {
        let path = path.as_ref();
        let fullpath = App::cachedir().join(path);

        debug!("start write {:?} data to {:?}", path, fullpath.display());

        use crate::helper::filehelper::sure_file_new;
        sure_file_new(&fullpath); //  确保文件夹已创建, 确保文件是最新的

        if Self::_write(&fullpath, data).await.is_ok() {
            return Ok(Self::path2url(&fullpath)); // 返回网络地址
        }
        error!("error to write {} data", fullpath.display());

        Ok(String::new())
    }

    async fn _write(path: &PathBuf, data: Bytes) -> Result<(), std::io::Error> {
        tokio::fs::write(&path, data).await
    }
}

#[cfg(test)]
mod tests {
    use super::FileSaver;
    use super::SUFFIX_ROUTER;
    use crate::app::App;

    #[test]
    fn test_u2p() {
        let url = format!("{}{}", SUFFIX_ROUTER, "/middledir/b.txt");
        let a = FileSaver::url2path(&App::cachedir(), &url);
        println!("{} => {}", url, a.display());

        let url = format!("{}{}", SUFFIX_ROUTER, "/file.txt");
        let a = FileSaver::url2path(&App::cachedir(), &url);
        println!("{} => {}", url, a.display());
    }

    #[test]
    fn test_p2u() {
        let mut path = App::cachedir();
        path.push("test/a.txt");

        let url = FileSaver::path2url(&path);

        println!("{} => {}", path.display(), url);

        let mut path = App::datadir();
        path.push("a.txt");

        let url = FileSaver::path2url(&path);

        println!("{} => {}", path.display(), url);
    }
}
