use axum::{
    http::{header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue, Request},
    middleware::{self, Next},
    response::IntoResponse,
    Router,
};

use log::info;

use super::netlog;

const ORGIN: &'static str = "http://localhost:5173";

/**
 * 路由
 */
pub(crate) fn create_router() -> Router {
    Router::new()
        .nest("/t", super::routertest::router2test()) // test
        .nest("/f", super::routerfile::router2file()) // 文件/图片
        .merge(super::routerfile::router2assets()) // 静态资源
        // 统一中间件
        // 添加ACCESS_CONTROL_ALLOW_ORIGIN
        .layer(middleware::from_fn(access_origin))
        // 添加日志输出
        .layer(middleware::from_fn(netlog::net_log))
}

/**
 * 添加ACCESS_CONTROL_ALLOW_ORIGIN
 */
async fn access_origin<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let mut res = next.run(req).await;

    if let Ok(value) = HeaderValue::from_str(ORGIN) {
        res.headers_mut().insert(ACCESS_CONTROL_ALLOW_ORIGIN, value);
        return res;
    }
    info!("error to add access origin");
    res
}
