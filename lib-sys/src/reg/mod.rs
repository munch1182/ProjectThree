//!
//! 对注册表项的值进行增删改查
//!
//! ```rust
//! let reg = sys::reg::RegHelper::new(sys::reg::HKEY_USERS);
//! let netset = reg.open("Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings")?;
//! let proxy = netset.read("ProxyEnable")?;
//! let _enable: u32 = proxy.try_into()?;
//!```
//!
use liblib::{str, Result};
use std::{ffi::OsStr, ptr};
pub use winapi::shared::minwindef::HKEY;
use winapi::{
    shared::minwindef::DWORD,
    um::winreg::{
        RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegQueryValueExW, RegSetValueExW, REGSAM,
    },
};
mod regtype;
pub use regtype::*;
/// 将reg类型重新导出
macro_rules! re_exports {
    ([$($v:ident),*]) => {
        $(pub use winapi::um::winreg::$v;)*
    };
}

re_exports!([
    HKEY_CURRENT_USER,
    HKEY_CLASSES_ROOT,
    HKEY_LOCAL_MACHINE,
    HKEY_USERS,
    HKEY_PERFORMANCE_DATA,
    HKEY_PERFORMANCE_TEXT,
    HKEY_PERFORMANCE_NLSTEXT,
    HKEY_CURRENT_CONFIG,
    HKEY_DYN_DATA,
    HKEY_CURRENT_USER_LOCAL_SETTINGS
]);

///
/// 注册表项对象
///
/// # Examples
///
/// ```rust
/// let reg = sys::reg::RegHelper::new(sys::reg::HKEY_USERS);
/// let netset = reg.open("Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings")?;
/// let proxy = netset.read("ProxyEnable")?;
/// let _enable: u32 = proxy.try_into()?;
/// ```
///
#[derive(Debug)]
pub struct RegHelper {
    hkey: HKEY,
}

macro_rules! sys_result {
    ($e:expr) => {
        match $e {
            0 => Ok(true),
            err => Err(std::io::Error::from_raw_os_error(err as i32)),
        }
    };
}

pub use winapi::um::winnt::{
    KEY_ALL_ACCESS, KEY_CREATE_LINK, KEY_CREATE_SUB_KEY, KEY_ENUMERATE_SUB_KEYS, KEY_EXECUTE,
    KEY_NOTIFY, KEY_QUERY_VALUE, KEY_READ, KEY_SET_VALUE, KEY_WOW64_32KEY, KEY_WOW64_64KEY,
    KEY_WOW64_RES, KEY_WRITE,
};

macro_rules! value2enum {
    ($t:ident, $doc: expr => [$($v:ident),*]) => (
        #[doc=$doc]
        #[allow(non_camel_case_types)]
        #[derive(Debug,Clone,PartialEq)]
        pub enum $t{
            $(  ///
                $v = winapi::um::winnt::$v as isize
            ),*
        }
    )
}

value2enum!(RegType,"注册表的值的类型的枚举, 从winapi::um::winnt转换而来" => [
    REG_NONE,
    REG_SZ,
    REG_EXPAND_SZ,
    REG_BINARY,
    REG_DWORD,
    REG_DWORD_BIG_ENDIAN,
    REG_LINK,
    REG_MULTI_SZ,
    REG_RESOURCE_LIST,
    REG_FULL_RESOURCE_DESCRIPTOR,
    REG_RESOURCE_REQUIREMENTS_LIST,
    REG_QWORD
]);

///
/// 注册表值对象
///
#[derive(Debug)]
pub struct RegValue {
    /// 注册表值的数据
    pub bytes: Vec<u8>,
    /// 注册表值的对象
    pub vtype: RegType,
}

///
/// 操作注册表
///
/// 使用时, 要注意权限, 即REGSAM
///
///
impl RegHelper {
    /// new
    /// [HKEY]
    pub const fn new(reg: HKEY) -> Self {
        Self { hkey: reg }
    }

    /// 默认用于全部权限
    pub fn open<P: AsRef<OsStr>>(&self, path: P) -> Result<RegHelper> {
        self.open_with(path, KEY_ALL_ACCESS)
    }

    /// 打开注册表时要注意权限, 如果无权限无法操作
    pub fn open_with<P>(&self, path: P, perms: REGSAM) -> Result<RegHelper>
    where
        P: AsRef<OsStr>,
    {
        let name = str::to_u16(path);
        let mut newhkey: HKEY = ptr::null_mut();
        unsafe {
            let status = RegOpenKeyExW(self.hkey, name.as_ptr(), 0, perms, &mut newhkey);
            sys_result!(status)?;
            Ok(RegHelper::new(newhkey))
        }
    }

    /// 读取值
    pub fn read<P: AsRef<OsStr>>(&self, key: P) -> Result<RegValue> {
        let key = str::to_u16(key);
        let mut dlen: DWORD = 0;
        let mut dtype: DWORD = 0;
        unsafe {
            // 先查询长度
            let status = RegQueryValueExW(
                self.hkey,
                key.as_ptr(),
                ptr::null_mut(),
                &mut dtype,
                ptr::null_mut(),
                &mut dlen,
            );
            sys_result!(status)?;
            let mut bytes = vec![0u8; dlen as usize];
            // 再查询值
            let status = RegQueryValueExW(
                self.hkey,
                key.as_ptr(),
                ptr::null_mut(),
                &mut dtype,
                bytes.as_mut_ptr(),
                &mut dlen,
            );
            sys_result!(status)?;
            let vtype: RegType = std::mem::transmute(dtype as u8); // 类型强转
            Ok(RegValue { bytes, vtype })
        }
    }

    /// 新加或者修改一对键值
    pub fn set<P: AsRef<OsStr>>(&self, key: P, value: &RegValue) -> Result<()> {
        let name = str::to_u16(key);
        let dtype = value.vtype.clone() as DWORD;
        let len = value.bytes.len() as u32;
        use winapi::shared::minwindef::BYTE;
        let data = value.bytes.as_ptr() as *const BYTE;
        unsafe {
            let status = RegSetValueExW(self.hkey, name.as_ptr(), 0, dtype, data, len);
            sys_result!(status)?;
            Ok(())
        }
    }

    /// 删除键值对
    pub fn del<P: AsRef<OsStr>>(&self, key: P) -> Result<()> {
        let name = str::to_u16(key);
        unsafe {
            let status = RegDeleteValueW(self.hkey, name.as_ptr());
            sys_result!(status)?;
            Ok(())
        }
    }

    fn close(&mut self) -> Result<()> {
        if self.hkey >= HKEY_CLASSES_ROOT {
            return Ok(());
        }
        sys_result!(unsafe { RegCloseKey(self.hkey) as DWORD })?;
        Ok(())
    }
}

impl Drop for RegHelper {
    fn drop(&mut self) {
        self.close().unwrap_or(())
    }
}
