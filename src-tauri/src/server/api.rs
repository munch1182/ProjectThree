use std::process::Command;

use crate::app::App;

use super::basebean::BaseBean;
use axum::{extract::Path, routing::get, Json, Router};
use log::info;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/cmd/:type/:content", get(cmd_bean))
}

async fn root() -> Json<BaseBean<i64>> {
    Json(BaseBean::success().data(App::get_server_start_time()))
}

fn wrapper<D: Copy>(bean: BaseBean<D>) -> Json<BaseBean<D>> {
    Json(bean)
}

#[warn(unused_variables)]
async fn cmd_bean(Path((t, args)): Path<(u16, String)>) -> Json<BaseBean<&'static str>> {
    wrapper(cmd(t, args))
}

fn cmd(t: u16, args: String) -> BaseBean<&'static str> {
    let program = get_cmd_program(t);
    let arg = format!("http://{}", args);

    let execute = Command::new(&program).arg(&arg).output();

    if let Ok(output) = execute {
        info!("cmd: '{} {}' : {}", program, arg, output.status);

        if let Some(1) = output.status.code() {
            return BaseBean::success().data("success");
        }
    };
    BaseBean::error1()
}

fn get_cmd_program(t: u16) -> String {
    match t {
        1 => String::from("explorer"),
        _ => String::from("start"),
    }
}
