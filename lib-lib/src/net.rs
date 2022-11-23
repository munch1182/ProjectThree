//!
//! 网络相关
//!
use crate::{err, err_to, Result};

/// 新建一个随机可用端口的地址并保存
pub struct NetAddr {
    port: u16,
}

impl NetAddr {
    /// 返回一个随机可用端口的127.0.0.1地址
    pub fn new() -> Result<Self> {
        let addr: std::net::SocketAddr = err_to!("0.0.0.0:00".parse())?;
        Ok(Self { port: addr.port() })
    }

    /// 返回全地址
    pub fn addr(&self) -> String {
        format!("http://localhost:{}", self.port)
    }
}
