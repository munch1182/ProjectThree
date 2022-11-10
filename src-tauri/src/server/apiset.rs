use super::api::wrapper;
use super::basebean::BaseBean;
use crate::proxy;
use axum::extract::Path;
use axum::Json;

pub async fn set_bean(Path((t, cmd)): Path<(u16, u8)>) -> Json<BaseBean<&'static str>> {
    wrapper(set(t, cmd))
}

fn set(_t: u16, _cmd: u8) -> BaseBean<&'static str> {
    proxy::start_proxy_server();
    BaseBean::error1()
}
