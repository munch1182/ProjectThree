//!
//! 对image进行操作
//!
//! ```ignore
//! # fn main() -> lib::Result<()> {
//!     // 需要真实存在该文件
//!     let req = r#"
//!     {
//!         "url":"/a/a.png",
//!         "operate":[
//!             [{"resize":{"w":16,"h":16}},{"resize":{"w":48,"h":48}},{"resize":{"w":56,"h":56}},{"convert":0}],
//!             [{"resize":{"w":16,"h":16}}],
//!             [{"resize":{"w":32,"h":32}}],
//!             [{"resize":{"w":48,"h":48}}]
//!         ]
//!     }"#;
//!     let mut req: ImageRequest = lib::json_from_str(req)?;
//!     let _result = req.operate()?;
//! #   Ok(())
//! # }
//! ```
//!
mod imgbean;
mod operate;

pub use imgbean::*;
