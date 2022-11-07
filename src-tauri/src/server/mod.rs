mod api;
mod basebean;

use crate::app::App;
use crate::appresult::{AppError, AppResult};
use axum::response::Response;
use axum::{
    body::{Body, Bytes},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
};
use log::{debug, error, info};

/**
 * 创建服务
 */
pub fn create_server() {
    info!("create tokio env.");
    if let Err(e) = create_tokio() {
        error!("{}", e.msg)
    }
}

fn create_tokio() -> AppResult<()> {
    let tokio = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    tokio.block_on(create_axum())
}

async fn create_axum() -> AppResult<()> {
    info!("create axum.");

    let app = api::create_router().layer(middleware::from_fn(log_req_res));

    let result = axum::Server::bind(&"0.0.0.0:0".parse()?).serve(app.into_make_service()); //随机可用端口

    let addr = format!("localhost:{}", result.local_addr().port());

    info!("start server: http://{}.", addr);

    App::set_server_addr(addr); //保存服务器地址

    if let Err(e) = result.await {
        info!("error start server.");
        return Err(AppError { msg: e.to_string() });
    }
    Ok(())
}

/**
 * 打印请求和响应的实际数据
 */
// todo 过滤掉文件请求
async fn log_req_res(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    log_req(&req);

    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print(body).await?;

    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    log_res(&res);

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print(body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

fn log_req(req: &Request<Body>) {
    let method = req.method();
    let uri = req.uri();

    debug!("{} {}", method, uri);

    let header = req.headers();

    for (key, value) in header {
        debug!("{}: {:#?}", key, value);
    }
}

fn log_res(res: &axum::response::Response) {
    let header = res.headers();

    for (key, value) in header {
        debug!("{}: {:?}", key, value);
    }
}

async fn buffer_and_print<B>(body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(res) => res,
        Err(err) => {
            return Err((StatusCode::BAD_REQUEST, format!("error:{}", err)));
        }
    };
    if let Ok(str) = std::str::from_utf8(&bytes) {
        debug!("{}", str);
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {

    use super::create_server;
    use crate::log;

    /**
     * 测试服务器
     */
    #[test]
    fn test_server() {
        log::init_log();
        create_server();
    }
}
