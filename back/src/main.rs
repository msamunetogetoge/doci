use axum::{
    extract,
    http::StatusCode,
    response,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::net::SocketAddr;

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
        .route("/add", post(register_page));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
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
    Json(
        get_web_page(&pool, page_info.app_id, &page_info.page_path)
            .await
            .is_ok(),
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
