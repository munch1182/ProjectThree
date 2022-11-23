//!
//! 与特性无关的语法库
//!
#![deny(missing_docs)]

// cargo doc --no-deps --open
mod macros; // 宏不需要使用pub导出

pub mod file;
pub mod net;
pub mod str;

/// 重导出lazy_static
pub use lazy_static::lazy_static;
/// 重导出log
pub use log::{debug, error, info};

/// 重导出serde_json
#[cfg(feature = "json")]
pub use serde_json::{from_str as json_from_str, to_string as json_to_string};
/// 重导出serde_repr
#[cfg(feature = "json")]
pub use serde_repr::{Deserialize_repr, Serialize_repr};

#[cfg(feature = "win")]
pub mod win;

/// 统一Error, 或许以后会改
pub type Error = std::io::Error;
/// 统一Error
///
/// [Error]
pub type Result<T> = std::result::Result<T, Error>;
