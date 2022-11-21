pub mod log;
pub mod macros; // 宏
pub mod str;

pub use lazy_static::lazy_static;

pub use serde_json::{from_str, to_string};
pub use serde_repr::{Deserialize_repr, Serialize_repr};

pub type Error = std::io::Error;
pub type Result<T> = std::result::Result<T, Error>;
