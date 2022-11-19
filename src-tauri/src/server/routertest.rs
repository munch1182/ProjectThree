use super::bean::baseres::NetBB as BB;
use crate::app::{App, ServerAddr};
use axum::{extract::Path, routing::get, Json, Router};

/**
 * 路由, /t+..
 */
pub(crate) fn router2test() -> Router {
    Router::new()
        .route("/t/:any", get(test))
        .route("/t", get(time))
}

async fn test(Path(str): Path<String>) -> String {
    str
}

async fn time() -> Json<BB<ServerAddr>> {
    if let Some(server) = App::get_server() {
        return BB::success(server).to();
    }
    BB::fail().to()
}
