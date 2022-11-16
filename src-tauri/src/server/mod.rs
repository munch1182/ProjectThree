mod netlog;
mod response;
mod router;
mod routertest;

use crate::app::App;
use anyhow::{anyhow, Result};
use log::{error, info};

pub(crate) fn create_server() {
    if let Err(e) = create_tokio() {
        error!("error create_server: {}.", e);
    }
}

fn create_tokio() -> Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(create_axum())
}

async fn create_axum() -> Result<()> {
    let app = router::create_router();

    let result = axum::Server::bind(&"0.0.0.0:00".parse()?).serve(app.into_make_service());

    let addr = format!("http://localhost:{}", result.local_addr().port());

    info!("start server: {}.", addr);
    App::set_server(addr); //保存服务器地址

    if let Err(e) = result.await {
        return Err(anyhow!("error to create axum: {}", e));
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::create_server;
    use crate::helper;

    /**
     * 测试服务器
     */
    #[test]
    fn test_server() {
        helper::init();
        create_server();
    }
}
