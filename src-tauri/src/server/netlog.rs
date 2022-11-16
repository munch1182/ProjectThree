use axum::{
    body::Bytes,
    http::{header::CONTENT_TYPE, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use hyper::{Body, HeaderMap, Response};
use log::debug;

use crate::app::App;

fn log_net(any: String) {
    debug!("{any}")
}

/**
 * 在debug中打印详细请求
 */
pub async fn net_log(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // req
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let url = App::get_server_addr().unwrap_or(String::from(""));
    // req: url
    log_net(format!("{} {}{} ===>", method, url, uri));
    // req: header
    let headers = req.headers();
    for (k, v) in headers {
        log_net(format!("{}: {}", k, v.to_str().unwrap_or("")));
    }

    let need = need_print(headers);
    let (parts, body) = req.into_parts();
    // req: body
    let bytes = buffer_and_print(need, body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));
    let res = next.run(req).await;

    // res
    log_net(format!("<=== {} {}{}", method, url, uri));
    let headers = res.headers();

    // res: header
    for (k, v) in headers {
        log_net(format!("{}: {}", k, v.to_str().unwrap_or("")));
    }

    let need = need_print(headers);
    let (parts, body) = res.into_parts();
    // res: body
    let bytes = buffer_and_print(need, body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));
    Ok(res)
}

fn need_print(headers: &HeaderMap) -> bool {
    if let Some(c) = headers.get(CONTENT_TYPE) {
        let t = c.to_str().unwrap_or("");
        if let Ok(m) = t.parse::<mime::Mime>() {
            return match m.type_() {
                mime::JSON | mime::TEXT | mime::PLAIN | mime::APPLICATION => true,
                _ => false,
            };
        }
    }
    true
}

// https://github.com/tokio-rs/axum/blob/main/examples/print-request-response/src/main.rs
async fn buffer_and_print<B>(need_print: bool, body: B) -> Result<Bytes, (StatusCode, String)>
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
    if need_print {
        if let Ok(str) = std::str::from_utf8(&bytes) {
            log_net(str.to_string());
        }
    }
    Ok(bytes)
}
