use axum::{
    extract,
    http::StatusCode,
    response,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{fs, net::SocketAddr};

use tracing_subscriber::fmt;

pub mod models;
use crate::models::{schemas::*, utils::*};

pub mod users;
use crate::users::auth::*;

#[tokio::main]
async fn main() {
    // initing
    models_init();

    // tracing のフォーマット作成
    let format = fmt::format().with_level(true).with_target(true).compact();
    // initialize tracing
    tracing_subscriber::fmt().event_format(format).init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/check", post(check_exist_page))
        .route("/get_hierarchy", post(get_hierarchy))
        .route("/add", post(register_page))
        .route("/edit", post(get_page))
        .route("/login", post(login))
        .route("/user", post(add_user))
        .route("/user", put(edit_user_info))
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
    let b = get_web_page(&pool, page_info.app_id, &page_info.page_path)
        .await
        .is_ok();
    println!("check_exist_page, data = {:?},{}", &page_info, &b);
    Json(b)
}

// web_pages, page_hierarchyにデータを作成する。dbに既にデータが存在すれば、ファイルだけ更新する
async fn register_page(extract::Json(page_info): extract::Json<WebPageInfo>) -> impl IntoResponse {
    let pool = get_conn().await;
    let page = add_web_page(&pool, page_info).await;
    match page {
        Ok(()) => StatusCode::CREATED,
        Err(err) => {
            println!("{}", err.to_string());
            tracing::error!("{}", err.to_string());
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct HierarchyInfo {
    id: Option<i64>,
    app_id: i64,
    parent_path: String,
    depth: i32,
}

async fn get_hierarchy(
    extract::Json(info): extract::Json<HierarchyInfo>,
) -> response::Json<Vec<HierarchyTS>> {
    let pool = get_conn().await;
    if info.id == None {
        Json(get_page_structure(&pool, info.app_id, info.parent_path, info.depth).await)
    } else {
        Json(get_page_structure_from_id(&pool, info.id.unwrap()).await)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct HierarchyID {
    id: i64,
}

async fn delete_page(extract::Json(hierarchy_id): extract::Json<HierarchyID>) -> impl IntoResponse {
    let pool = get_conn().await;
    if let Err(err) = delete_pages(&pool, hierarchy_id.id).await {
        tracing::info!("In delete_page error occured:{}", err);
        println!("{}", err.to_string());
        return StatusCode::INTERNAL_SERVER_ERROR;
    };
    StatusCode::ACCEPTED
}

#[derive(Serialize, Deserialize, Debug)]
struct Page {
    page_path: String,
    md: String,
}
// ドキュメント(hoge.md)の内容を返す
async fn get_page(extract::Json(hierarchy_id): extract::Json<HierarchyID>) -> extract::Json<Page> {
    let pool = get_conn().await;
    let (app_id, page_path) = get_web_page_info(&pool, hierarchy_id.id).await.unwrap();
    let web_page = get_web_page(&pool, app_id, &page_path).await.unwrap();
    if let Ok(res) = fs::read_to_string(&web_page.file_path) {
        Json(Page {
            page_path: web_page.page_path,
            md: res,
        })
    } else {
        tracing::error!("ファイルを読み込むのに失敗した");
        Json(Page {
            page_path: String::from(""),
            md: String::from(""),
        })
    }
}

// loginに使うstruct
#[derive(Serialize, Deserialize, Debug)]

struct LoginInfo {
    mailaddress: String,
    password: String,
}

// id,pass に該当があればtrue,なければfalse
async fn login(extract::Json(info): extract::Json<LoginInfo>) -> extract::Json<bool> {
    let pool = get_conn().await;
    let canlogin = find_user(&pool, info.mailaddress, info.password).await;
    Json(canlogin)
}

// dbにユーザーを登録する
async fn add_user(extract::Json(user): extract::Json<UserInfo>) -> impl IntoResponse {
    let pool = get_conn().await;
    if let Err(e) = signup_user(&pool, user).await {
        tracing::error!("In add_usererror occured:{}", e);
        return StatusCode::BAD_REQUEST;
    };
    StatusCode::CREATED
}

// ユーザー情報を編集する
async fn edit_user_info(extract::Json(user): extract::Json<UserInfo>) -> impl IntoResponse {
    let pool = get_conn().await;
    if let Err(e) = edit_user(&pool, user).await {
        tracing::error!("In edit_user_info error occured:{}", e);
        return StatusCode::BAD_REQUEST;
    };
    StatusCode::ACCEPTED
}
