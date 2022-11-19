use axum::{http::HeaderValue, middleware, Router};

use hyper::{header, Method};
use tower_http::cors::CorsLayer;

use super::netlog;

const ORIGIN: &'static str = "http://localhost:5173";

/**
 * 路由
 */
pub(crate) fn create_router() -> Router {
    Router::new()
        .nest("/t", super::routertest::router2test()) // test
        .nest("/f", super::routerfile::router2file()) // 文件/图片
        .merge(super::routerfile::router2assets()) // 静态资源
        // 统一中间件
        // 添加日志输出
        .layer(middleware::from_fn(netlog::net_log))
        // https://developer.mozilla.org/zh-CN/docs/Web/HTTP/CORS
        // 添加cors处理, 允许origin访问并自动通过options, 允许请求携带json
        .layer(
            CorsLayer::new()
                .allow_origin(ORIGIN.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([header::CONTENT_TYPE]),
        )
}
