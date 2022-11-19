use image::GenericImageView;
use log::error;

use crate::{app::App, server::bean::image::ImageInfo};
use std::{
    fs::{self, DirBuilder},
    path::{Path, PathBuf},
};

/**
 * 确保该路径可用
 * 如果传入的是一个文件夹, 则确保该文件夹已被创建
 * 如果传入的是一个文件, 则确保文件所在的文件夹已被创建
 *
 * 返回是否创建成功
 */
pub fn sure_path<P: AsRef<Path>>(p: P) -> bool {
    let mut tmp = p.as_ref().to_path_buf();
    // 如果是文件: 只有在已存在时才会返回true
    if tmp.is_file() {
        tmp.pop();
        // 如果有后缀名
    } else if let Some(_) = tmp.extension() {
        tmp.pop();
    }
    if !tmp.exists() {
        return DirBuilder::new().recursive(true).create(tmp).is_ok();
    }
    return true;
}

/**
 * 如果传入的是一个文件, 会先确保该文件所在文件夹路径存在, 且该文件已存在, 则会删除该文件
 */
pub fn sure_file_new<P: AsRef<Path>>(p: P) -> bool {
    let tmp = p.as_ref();
    sure_path(tmp);
    if tmp.exists() && tmp.is_file() {
        return fs::remove_file(tmp).is_ok();
    }
    return true;
}

pub fn image_read<P: AsRef<Path>>(p: P) -> anyhow::Result<ImageInfo> {
    let p = p.as_ref();

    if p.exists() {
        let i = image::open(p)?;
        if let Some(name) = p.file_name() {
            let name = name.to_os_string().to_str().unwrap_or("").to_string();
            let mut len = 0;
            if let Ok(m) = p.metadata() {
                len = m.len();
            }
            let path = crate::server::routerfile::FileSaver::path2url(&p);
            let (w, h) = i.dimensions();
            return Ok(ImageInfo::new(name, path, len, w, h));
        }
    }

    return Err(anyhow::anyhow!("error to read image"));
}

pub fn image2ico<P: AsRef<Path>>(p: P, size: u32) -> anyhow::Result<ImageInfo> {
    let p = p.as_ref();

    if !p.exists() {
        return Err(anyhow::anyhow!("file is not exosts"));
    }

    // 读取原图信息
    let origin_info = image_read(p)?;
    // 先进行更改大小
    let p = _image_resize(p, (size, size))?;

    println!("p:{}", p.display());
    // 读取文件
    let image_file = std::fs::File::open(&p)?;
    // 创建icon类
    let mut ico_dir = ico::IconDir::new(ico::ResourceType::Icon);

    // 写入图片
    let icon_image = ico::IconImage::read_png(image_file)?;
    let image_entry = ico::IconDirEntry::encode(&icon_image)?;
    ico_dir.add_entry(image_entry);

    let size_usize: usize = size.try_into()?;
    // 写入rgba
    let rgba_data = vec![std::u8::MAX; 4 * size_usize * size_usize];
    let rgba_image = ico::IconImage::from_rgba_data(size, size, rgba_data);
    let rgba_entry = ico::IconDirEntry::encode(&rgba_image)?;
    ico_dir.add_entry(rgba_entry);

    // 保存ico
    if let Some(name) = p.file_stem() {
        let mut ico_path = App::cachedir().join(name);
        ico_path.set_extension("ico");
        let path = ico_path.as_path();
        sure_file_new(path);
        let f = std::fs::File::create(path)?;
        let _ = ico_dir.write(f)?;
        let mut info = image_read(path)?;

        use crate::server::bean::image::ImageOperate::Ico;
        info.from(&origin_info, Ico);
        return Ok(info);
    }
    error!("error to create icon");
    return Err(anyhow::anyhow!("error to read image"));
}

/**
 * 只是进行更改尺寸, 返回的是新文件地址
 */
fn _image_resize<P: AsRef<Path>>(p: P, size: (u32, u32)) -> anyhow::Result<PathBuf> {
    let p = p.as_ref();
    if p.exists() {
        // 读取原图信息
        let origin_info = image_read(p)?;
        let dimen = origin_info.dimen;
        if dimen.w != size.0 || dimen.h != size.1 {
            let i = image::open(p)?;
            let n_img = i.resize(size.0, size.1, image::imageops::FilterType::Nearest);
            if let Some(newname) = &update_name(p, format!("{}x{}", size.0, size.1)) {
                sure_file_new(newname);
                let _ = n_img.save(newname)?;
                return Ok(newname.to_path_buf());
            }
        } else {
            return Ok(p.to_path_buf());
        }
    }
    Err(anyhow::anyhow!("error to resiez"))
}

/**
 * 更新文件名并返回该路径, 传入的p必须是文件路径
 *
 * 如: a.png => a_16x16.png
 */
fn update_name<P: AsRef<Path>>(p: P, add: String) -> Option<PathBuf> {
    let p = p.as_ref();
    let name = p.file_stem()?;
    let ext = p.extension()?;
    let mut newpath = PathBuf::from(p);
    newpath.pop();
    let newname = format!("{:#?}_{}", name, add).replace("\"", "");

    let mut newpath = newpath.join(Path::new(&newname));
    newpath.set_extension(ext);
    Some(newpath.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::update_name;
    use std::path::{Path, PathBuf};

    #[test]
    fn test() {
        let p: PathBuf = PathBuf::new().join("a").join(Path::new("b.txt"));

        let target = PathBuf::new().join("a").join(Path::new("b_16x16.txt"));
        assert_eq!(update_name(p, "16x16".to_string()), Some(target));
    }
}
