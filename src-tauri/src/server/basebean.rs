use serde::Deserialize as serde_D;
use serde::{
    de::{Deserialize, Deserializer},
    ser::Serializer,
    Serialize,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Code {
    Success,
    Fail(u8),
}

/**
 * 将code在序列化的时候将类型改为u8, Code::Success => 0, Code::Fail(code:u8) => code
 */
impl Serialize for Code {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Code::Success => serializer.serialize_u8(0),
            Code::Fail(code) => serializer.serialize_u8(*code),
        }
    }
}

/**
 * see Serialize
 */
impl<'de> Deserialize<'de> for Code {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = u8::deserialize(deserializer)?;
        if code == 0 {
            Ok(Code::Success)
        } else {
            Ok(Code::Fail(code))
        }
    }
}

#[derive(Debug, Serialize, serde_D, Clone, Copy)]
pub struct BaseBean<D: Copy> {
    pub code: Code, // 序列化时是u8类型
    pub data: Option<D>,
}

impl<D: Copy> BaseBean<D> {
    pub fn success() -> Self {
        BaseBean {
            code: Code::Success,
            data: None,
        }
    }

    pub fn error(code: u8) -> Self {
        BaseBean {
            code: Code::Fail(code),
            data: None,
        }
    }

    pub fn error1() -> Self {
        Self::error(1)
    }

    pub fn data(&mut self, data: D) -> Self {
        self.data = Some(data);
        *self
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_json() {
        let str = "{\"code\":0,\"data\":\"success\"}";
        let bean = BaseBean::success().data("success");
        let parse_bean = serde_json::from_str::<BaseBean<&str>>(str).unwrap();

        assert_eq!(bean.code, parse_bean.code);
        assert_eq!(bean.data, parse_bean.data);
        assert_eq!(serde_json::to_string(&parse_bean).unwrap(), str);

        println!("{:?}", bean);

        let str = "{\"code\":1}";
        let bean = BaseBean::error1();
        let parse_bean = serde_json::from_str::<BaseBean<&str>>(str).unwrap();

        assert_eq!(bean.code, parse_bean.code);
        assert_eq!(bean.data, parse_bean.data);

        let str = "{\"code\":1,\"data\":null}";
        let bean = BaseBean::error1();
        let parse_bean = serde_json::from_str::<BaseBean<&str>>(str).unwrap();

        assert_eq!(bean.code, parse_bean.code);
        assert_eq!(bean.data, parse_bean.data);
        assert_eq!(serde_json::to_string(&parse_bean).unwrap(), str);

        println!("{:?}", bean);
    }
}
