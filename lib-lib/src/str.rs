//!
//! 字符串相关
//!
use std::{os::windows::prelude::OsStrExt, slice};

/// 将字符转为u16集合, 适合写入window系统
pub fn to_u16<P: AsRef<std::ffi::OsStr>>(p: P) -> Vec<u16> {
    p.as_ref()
        .encode_wide()
        .chain(Some(0).into_iter()) // 只是在最后一位加上了0
        .collect()
}

/// 将u16字符转为u8字符
pub fn u16_to_u8(u: &[u16]) -> Vec<u8> {
    unsafe { slice::from_raw_parts(u.as_ptr() as *const u8, u.len() * 2).to_vec() }
}

/// 将字符转为u16, 再转为u8
///
/// [to_u16]
/// [u16_to_u8]
pub fn to_u8_code<P: AsRef<std::ffi::OsStr>>(p: P) -> Vec<u8> {
    u16_to_u8(&to_u16(p))
}
