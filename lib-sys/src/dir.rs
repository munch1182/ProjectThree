use std::{ffi::OsString, os::windows::prelude::OsStringExt, ptr};

use winapi::{
    shared::winerror,
    um::{combaseapi, knownfolders, shlobj, shtypes, winbase, winnt},
};

// pub fn roaming() -> Option<OsString> {
//     dir(&knownfolders::FOLDERID_RoamingAppData)
// }

// pub fn music() -> Option<OsString> {
//     dir(&knownfolders::FOLDERID_Music)
// }

// pub fn pictures() -> Option<OsString> {
//     dir(&knownfolders::FOLDERID_Pictures)
// }

// pub fn videos() -> Option<OsString> {
//     dir(&knownfolders::FOLDERID_Videos)
// }

// pub fn download() -> Option<OsString> {
//     dir(&knownfolders::FOLDERID_Downloads)
// }

// pub fn desktop() -> Option<OsString> {
//     dir(&knownfolders::FOLDERID_Desktop)
// }

// pub fn documents() -> Option<OsString> {
//     dir(&knownfolders::FOLDERID_Documents)
// }

// pub fn local() -> Option<OsString> {
//     dir(&knownfolders::FOLDERID_LocalAppData)
// }

///
/// 即user文件夹
///
pub fn user() -> Option<OsString> {
    dir(&knownfolders::FOLDERID_Profile)
}

// https://github.com/dirs-dev/dirs-sys-rs/blob/main/src/lib.rs
fn dir(rfid: shtypes::REFKNOWNFOLDERID) -> Option<OsString> {
    unsafe {
        let mut path_ptr: winnt::PWSTR = ptr::null_mut();
        let result = shlobj::SHGetKnownFolderPath(rfid, 0, ptr::null_mut(), &mut path_ptr);
        if result == winerror::S_OK {
            let len = winbase::lstrlenW(path_ptr) as usize;
            let path = std::slice::from_raw_parts(path_ptr, len);
            let ostr: OsString = OsStringExt::from_wide(path);
            combaseapi::CoTaskMemFree(path_ptr as *mut winapi::ctypes::c_void);
            Some(OsString::from(ostr))
        } else {
            combaseapi::CoTaskMemFree(path_ptr as *mut winapi::ctypes::c_void);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo.exe test -- dir::tests::test --exact --nocapture
    #[test]
    fn test() {
        // println!("----");
        // let dir = roaming();
        // println!("roaming_app_data: {:?}", dir);
        // assert!(dir.is_some());


        // let dir = local();
        // println!("local_app_data: {:?}", dir);
        // assert!(dir.is_some());

        // let dir = music();
        // println!("music: {:?}", dir);
        // assert!(dir.is_some());

        let dir = user();
        println!("user: {:?}", dir);
        assert!(dir.is_some());
    }
}
