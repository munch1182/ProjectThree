use super::response::baseres::NetBB as BB;
use axum::{extract::Multipart, routing::post, Json, Router};
use log::info;
/**
 * 路由, /i+..
 */
pub(crate) fn router2file() -> Router {
    Router::new().route("/u", post(receive_upload))
}

async fn receive_upload(mut part: Multipart) -> Json<BB<bool>> {
    while let Some(f) = part.next_field().await.unwrap_or(None) {
        let content_type = f.content_type().unwrap_or("");

        if content_type.is_empty() {
            // 必须要有类型
            return BB::file_req_err().to();
        }
        let want_name = f.name().unwrap_or(""); // 必须要有Content-Disposition
        if want_name.is_empty() {
            return BB::file_req_err().to();
        }
        let save_name = format!("{}", want_name);

        info!("save name:{}", save_name);

        if let Ok(data) = f.bytes().await {
            // 写入文件
            let reuslt = tokio::fs::write(&save_name, data)
                .await
                .map_err(|err| err.to_string());

            if let Err(e) = reuslt {
                info!("error to save file:{}", e);
                return BB::file_res_err().to();
            }
            return BB::success(true).to();
        }
    }

    BB::other(2).to()
}
