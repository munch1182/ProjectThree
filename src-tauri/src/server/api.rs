use crate::app::App;

use super::apidoc::query_apidoc;
use super::basebean::BaseBean;
use super::{apicmd::cmd_bean, apiset::set_bean};
use axum::{routing::get, Json, Router};

pub fn wrapper<D>(bean: BaseBean<D>) -> Json<BaseBean<D>>
where
    D: Clone + Copy,
{
    Json(bean)
}

async fn root() -> Json<BaseBean<i64>> {
    Json(*BaseBean::success().data(App::get_server_start_time()))
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/cmd/:type/:content", get(cmd_bean))
        .route("/set/:type/:cmd", get(set_bean))
        .route("/apidoc/query", get(query_apidoc))
}
