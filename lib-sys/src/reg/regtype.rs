use std::{ffi::OsString, os::windows::prelude::OsStringExt};

use super::{RegType, RegValue};

pub trait ToRegValue: Sized {
    fn to_reg_value(&self) -> RegValue;
}

/// any => RegValue
macro_rules! to_reg_value_sz {
    ($t:ty$(,$l:lifetime)*) => {
        impl<$($l,)*> ToRegValue for $t {
            fn to_reg_value(&self) -> RegValue{
                use lib::str::to_u8_code;
                RegValue {
                    bytes: to_u8_code(self),
                    vtype: RegType::REG_SZ
                }
            }
        }
    };
}

to_reg_value_sz!(String);
to_reg_value_sz!(&'a str, 'a);
to_reg_value_sz!(std::ffi::OsString);
to_reg_value_sz!(&'a std::ffi::OsStr, 'a);

macro_rules! to_reg_value_multi_sz {
    ($t:ty$(,$l:lifetime)*) => {
        impl<$($l,)*> ToRegValue for Vec<$t> {
            fn to_reg_value(&self) -> RegValue {
                use lib::str::{to_u16,u16_to_u8};
                let mut os_str = self.into_iter()
                .map(to_u16)
                .collect::<Vec<_>>()
                .concat();
                os_str.push(0);
                RegValue {
                    bytes: u16_to_u8(&os_str),
                    vtype: RegType::REG_MULTI_SZ
                }
            }
        }
    };
}

to_reg_value_multi_sz!(String);
to_reg_value_multi_sz!(&'a str, 'a);
to_reg_value_multi_sz!(std::ffi::OsString);
to_reg_value_multi_sz!(&'a std::ffi::OsStr, 'a);

impl ToRegValue for u32 {
    fn to_reg_value(&self) -> RegValue {
        let bytes: Vec<u8> =
            unsafe { std::slice::from_raw_parts((self as *const u32) as *const u8, 4).to_vec() };
        RegValue {
            bytes,
            vtype: RegType::REG_DWORD,
        }
    }
}

impl ToRegValue for u64 {
    fn to_reg_value(&self) -> RegValue {
        let bytes: Vec<u8> =
            unsafe { std::slice::from_raw_parts((self as *const u64) as *const u8, 8).to_vec() };
        RegValue {
            bytes,
            vtype: RegType::REG_QWORD,
        }
    }
}

/// RegValue ==> any
impl TryInto<String> for RegValue {
    type Error = lib::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        match self.vtype {
            RegType::REG_SZ | RegType::REG_EXPAND_SZ | RegType::REG_MULTI_SZ => {
                let words = unsafe {
                    std::slice::from_raw_parts(
                        self.bytes.as_ptr() as *const u16,
                        self.bytes.len() / 2,
                    )
                };
                let mut s = String::from_utf16_lossy(words);
                while s.ends_with('\u{0}') {
                    s.pop();
                }
                if self.vtype == RegType::REG_MULTI_SZ {
                    return Ok(s.replace('\u{0}', "\n"));
                }
                Ok(s)
            }
            _ => Err(lib::err!("cannot get type {:?} from regvalue", self.vtype)),
        }
    }
}

impl TryInto<Vec<String>> for RegValue {
    type Error = lib::Error;

    fn try_into(self) -> Result<Vec<String>, Self::Error> {
        match self.vtype {
            RegType::REG_MULTI_SZ => {
                let words = unsafe {
                    std::slice::from_raw_parts(
                        self.bytes.as_ptr() as *const u16,
                        self.bytes.len() / 2,
                    )
                };
                let mut s = String::from_utf16_lossy(words);
                while s.ends_with('\u{0}') {
                    s.pop();
                }
                let v = s.split('\u{0}').map(|x| x.to_owned()).collect();
                Ok(v)
            }
            _ => Err(lib::err!("cannot get type {:?} from regvalue", self.vtype)),
        }
    }
}

impl TryInto<OsString> for RegValue {
    type Error = lib::Error;

    fn try_into(self) -> Result<OsString, Self::Error> {
        match self.vtype {
            RegType::REG_MULTI_SZ => {
                let mut words = unsafe {
                    std::slice::from_raw_parts(
                        self.bytes.as_ptr() as *const u16,
                        self.bytes.len() / 2,
                    )
                };
                while let Some(0) = words.last() {
                    words = &words[0..words.len() - 1];
                }
                let v = OsString::from_wide(words);
                Ok(v)
            }
            _ => Err(lib::err!("cannot get type {:?} from regvalue", self.vtype)),
        }
    }
}

impl TryInto<Vec<OsString>> for RegValue {
    type Error = lib::Error;

    fn try_into(self) -> Result<Vec<OsString>, Self::Error> {
        match self.vtype {
            RegType::REG_MULTI_SZ => {
                let mut words = unsafe {
                    std::slice::from_raw_parts(
                        self.bytes.as_ptr() as *const u16,
                        self.bytes.len() / 2,
                    )
                };
                while let Some(0) = words.last() {
                    words = &words[0..words.len() - 1];
                }
                let v = words
                    .split(|ch| *ch == 0u16)
                    .map(|x| OsString::from_wide(x))
                    .collect();
                Ok(v)
            }
            _ => Err(lib::err!("cannot get type {:?} from regvalue", self.vtype)),
        }
    }
}

impl TryInto<u32> for RegValue {
    type Error = lib::Error;

    fn try_into(self) -> Result<u32, Self::Error> {
        match self.vtype {
            RegType::REG_DWORD => {
                let n = unsafe { *(self.bytes.as_ptr() as *const u32) };
                Ok(n)
            }
            _ => Err(lib::err!("cannot get type {:?} from regvalue", self.vtype)),
        }
    }
}

impl TryInto<u64> for RegValue {
    type Error = lib::Error;

    fn try_into(self) -> Result<u64, Self::Error> {
        match self.vtype {
            RegType::REG_QWORD => {
                let n = unsafe { *(self.bytes.as_ptr() as *const u64) };
                Ok(n)
            }
            _ => Err(lib::err!("cannot get type {:?} from regvalue", self.vtype)),
        }
    }
}
