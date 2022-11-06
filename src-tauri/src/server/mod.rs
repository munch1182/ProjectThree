mod api;
mod basebean;

use crate::app::App;
use crate::appresult::{AppError, AppResult};
use log::{error, info};

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

    let app = api::create_router();

    let result = axum::Server::bind(&"0.0.0.0:0".parse()?).serve(app.into_make_service()); //随机可用端口

    let addr = format!("localhost:{}", result.local_addr().port());

    info!("start server: {}.", addr);

    App::set_server_addr(addr); //保存服务器地址

    if let Err(e) = result.await {
        info!("error start server.");
        return Err(AppError { msg: e.to_string() });
    }
    Ok(())
}

// #[test]
// fn test_server() {
//     crate::log::init_log();
//     assert!(create_tokio().is_ok())
// }
