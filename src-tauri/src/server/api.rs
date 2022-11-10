use crate::app::App;

use super::apicmd::cmd_bean;
use super::apiset::set_bean;
use super::basebean::BaseBean;
use axum::{routing::get, Json, Router};

pub fn wrapper<D: Copy>(bean: BaseBean<D>) -> Json<BaseBean<D>> {
    Json(bean)
}

async fn root() -> Json<BaseBean<i64>> {
    Json(BaseBean::success().data(App::get_server_start_time()))
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/cmd/:type/:content", get(cmd_bean))
        .route("/set/:type/:cmd", get(set_bean))
}
