use axum::{Router, http::{StatusCode, Uri}, handler::Handler};
use liblib::{err, err_to, Result};

pub async fn create() -> Result<()> {
    axum::Server::bind(&err_to!("0.0.0.0:3000".parse())?).serve(Router::new().fallback(any.into_service()).into_make_service()).await.unwrap();
    Ok(())
}


async fn any(url:Uri) ->(StatusCode,String){
    println!("url:{}",url);
    (StatusCode::OK,"from net test".to_string())
}

pub fn create_in_thread() -> Result<()>{
    tokio::runtime::Builder::new_multi_thread().enable_all().build()?.block_on(create())
}