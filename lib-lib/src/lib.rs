pub mod log;
pub mod macros; // 宏

pub use lazy_static::lazy_static;

pub use serde_json::{from_str, to_string};
pub use serde_repr::{Deserialize_repr, Serialize_repr};
