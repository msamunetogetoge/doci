use axum::{
    extract,
    http::StatusCode,
    response,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::{net::SocketAddr};
use serde::{Deserialize, Serialize};


pub mod models;
use crate::models::{utils::*, schemas::*};

#[tokio::main]
async fn main() {
    // initing
    models_init();

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/check", post(check_exist_page))
        .route("/get_hierarchy", post(get_hierarchy))
        .route("/add", post(register_page))
        .route("/delete", post(delete_page));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// if exist -> return true
async fn check_exist_page(
    extract::Json(page_info): extract::Json<WebPageInfo>,
) -> response::Json<bool> {
    let pool = get_conn().await;
    let b = get_web_page(&pool, page_info.app_id, &page_info.page_path).await.is_ok();
    println!("check_exist_page, data = {:?},{}",&page_info, &b);
    Json(
        b
    )
}

// 作れるならデータを作成する
async fn register_page(extract::Json(page_info): extract::Json<WebPageInfo>) -> impl IntoResponse {
    let pool = get_conn().await;
    let page = add_web_page(&pool, page_info).await;
    match page {
        Ok(()) => StatusCode::CREATED,
        Err(err) => {
            println!("{}", err.to_string());
            // logger.Error(err.to_string());
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(Serialize,Deserialize, Debug)]
struct HierarchyInfo{
    id: Option<i64>,
    app_id: i64,
    parent_path: String,
    depth: i32,
}

async fn get_hierarchy(extract::Json(info): extract::Json<HierarchyInfo>) -> response::Json<Vec<HierarchyTS>>{
    let pool = get_conn().await;
    if info.id == None{
        Json(get_page_structure(&pool,info.app_id, info.parent_path, info.depth).await)
    }
    else{
        Json(get_page_structure_from_id(&pool,info.id.unwrap()).await)
    }
}

#[derive(Serialize,Deserialize, Debug)]
struct HierarchyID{
    id: i64,
}


async fn delete_page(extract::Json(hierarchy_id):extract::Json<HierarchyID>) ->  impl IntoResponse {
    let pool = get_conn().await;
    if let Err(err) = delete_pages(&pool, hierarchy_id.id).await{
        tracing::info!("In delete_page error occured:{}", err);
        println!("{}", err.to_string());
        return StatusCode::INTERNAL_SERVER_ERROR;
    };
    StatusCode::ACCEPTED
}
