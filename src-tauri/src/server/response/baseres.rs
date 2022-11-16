use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetCode {
    Success,
    Other(u16), // 前1000都是保留数字
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct NetBB<T> {
    code: NetCode,
    data: Option<T>,
}

impl<T> NetBB<T> {
    /**
     * 返回携带数据的成功
     */
    pub fn success(data: T) -> Self {
        Self {
            code: NetCode::Success,
            data: Some(data),
        }
    }

    /**
     * 返回无数据的成功
     */
    // pub fn success_no_data() -> Self {
    //     Self {
    //         code: NetCode::Success,
    //         data: None,
    //     }
    // }

    /**
     * 返回其它
     */
    pub fn other(code: u16) -> Self {
        if code < 1000 {
            panic!("other code cannot be smaller 1000")
        }
        Self {
            code: NetCode::Other(code),
            data: None,
        }
    }

    pub fn fail() -> Self {
        Self::other(1)
    }

    pub fn to(self) -> axum::Json<Self> {
        axum::Json(self)
    }
}

/**
 * 将code在序列化的时候将类型改为u8, Code::Success => 0, Code::Fail(code:u8) => code
 */
impl Serialize for NetCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NetCode::Success => serializer.serialize_u16(0),
            NetCode::Other(code) => serializer.serialize_u16(*code),
        }
    }
}

/**
 * see Serialize
 */
impl<'de> Deserialize<'de> for NetCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = u16::deserialize(deserializer)?;
        if code == 0 {
            Ok(NetCode::Success)
        } else {
            Ok(NetCode::Other(code))
        }
    }
}
