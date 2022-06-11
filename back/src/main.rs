use axum::{
    extract,
    http::StatusCode,
    response,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{fs, net::SocketAddr};

use tracing_subscriber::fmt;

pub mod models;
use crate::models::{schemas::*, utils::*};

pub mod users;
use crate::users::auth::*;
use crate::users::doc::*;

pub mod errors;

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
        // .route("/check", post(check_exist_page))
        .route("/get_hierarchy", post(get_hierarchy))
        .route("/page", post(register_page))
        .route("/doc", post(create_doc))
        .route("/doc/:app_id/path/:page_path", get(try_get_page))
        .route("/doc/:user_id", get(get_doc_infos))
        .route("/page/:hierarchy_id", get(get_page))
        .route("/login", post(login))
        .route("/user/:user_name", get(get_user))
        .route("/user", post(add_user))
        .route("/user", put(edit_user_info))
        .route("/page/:hierarchy_id", delete(delete_page));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

/// もしもデータが存在すればデータを返す。
/// なければNoneを返す
async fn try_get_page(
    extract::Path((app_id, page_path)): extract::Path<(i64, String)>,
) -> impl IntoResponse
// response::Json<Option<WebPages>>
{
    let pool = get_conn().await;
    let web_pages_or_none = get_web_page(&pool, app_id, &page_path).await;
    match web_pages_or_none {
        Ok(web_page) => (StatusCode::OK, Json(Some(web_page))),
        Err(e) => {
            tracing::error!("In try_get_page: error occured error:,{}", e);
            (StatusCode::BAD_REQUEST, Json(None))
        }
    }
}

/// web_pages, page_hierarchyにデータを作成する。
/// dbに既にデータが存在すれば、ファイルだけ更新する
async fn register_page(extract::Json(page_info): extract::Json<WebPageInfo>) -> impl IntoResponse {
    let pool = get_conn().await;
    let is_page_added = add_web_page(&pool, page_info).await;
    match is_page_added {
        Ok(()) => StatusCode::CREATED,
        Err(err) => {
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

///もらった階層構造のデータの子供全てのページ情報を取得する。
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

///web_pages, page_hierarchy からデータを削除する。
///成功すればステータスコード202, 失敗すれば500を返す
async fn delete_page(extract::Path(hierarchy_id): extract::Path<i64>) -> impl IntoResponse {
    let pool = get_conn().await;
    if let Err(err) = delete_pages(&pool, hierarchy_id).await {
        tracing::error!("In delete_page error occured:{}", err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    };
    StatusCode::ACCEPTED
}

#[derive(Serialize, Deserialize, Debug)]
struct Page {
    page_path: String,
    md: String,
}
///ドキュメント(hoge.md)の内容を返す
///ファイルを読み込めなかったときは空のデータを返す
async fn get_page(extract::Path(hierarchy_id): extract::Path<i64>) -> extract::Json<Page> {
    let pool = get_conn().await;
    let (app_id, page_path) = get_web_page_info(&pool, hierarchy_id).await.unwrap();
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

/// id,pass に該当があればtrue,なければfalse
async fn login(extract::Json(info): extract::Json<LoginInfo>) -> extract::Json<bool> {
    let pool = get_conn().await;

    let can_login = info.can_login(&pool).await;
    Json(can_login)
}

/// dbにユーザーを登録する
/// 登録出来れば201, 出来なければ400を返す
async fn add_user(extract::Json(user): extract::Json<UserInfo>) -> impl IntoResponse {
    let pool = get_conn().await;

    if let Err(e) = user.signup_user(&pool).await {
        tracing::error!("In add_user error occured:{}", e);
        return StatusCode::BAD_REQUEST;
    };
    StatusCode::CREATED
}

/// ユーザー情報を編集する
async fn edit_user_info(extract::Json(user): extract::Json<UserInfo>) -> impl IntoResponse {
    let pool = get_conn().await;
    if let Err(e) = user.edit_user(&pool).await {
        tracing::error!("In edit_user_info error occured:{}", e);
        return StatusCode::BAD_REQUEST;
    };
    StatusCode::ACCEPTED
}

/// username からユーザー情報を検索する。
/// もしもエラーが起きたらdefaultのデータを返す
async fn get_user(extract::Path(user_name): extract::Path<String>) -> extract::Json<UserInfo> {
    let pool = get_conn().await;
    let user_info = UserInfo {
        userid: None,
        username: user_name,
        mailaddress: None,
        password: None,
    };
    let user = user_info.get_user_info_from_name(&pool).await;
    match user {
        Ok(x) => Json(x),
        Err(e) => {
            tracing::error!("In get_user_info error occured:{}", e);
            let init_user_info: UserInfo = Default::default();
            Json(init_user_info)
        }
    }
}

/// user_id で指定されるユーザーが作成したdocumentを取得する
/// 失敗した場合はdefault を返す
async fn get_doc_infos(extract::Path(user_id): extract::Path<i64>) -> extract::Json<Vec<DocInfo>> {
    let pool = get_conn().await;
    let created_docs = get_created_doc(&pool, user_id).await;
    match created_docs {
        Ok(docs) => Json(docs),
        Err(e) => {
            tracing::error!("In get_doc_info error occured:{}", e);
            let init_doc_info: DocInfo = Default::default();
            Json(vec![init_doc_info])
        }
    }
}

/// applications にデータを作成する。
/// 成功したらステータスコード201,失敗したら400を返す
async fn create_doc(extract::Json(doc_info): extract::Json<DocInfo>) -> impl IntoResponse {
    let pool = get_conn().await;
    let res = doc_info.create_doc(&pool).await;
    match res {
        Ok(_) => StatusCode::CREATED,
        Err(e) => {
            tracing::error!("In create_doc error occured:{}", e);
            StatusCode::BAD_REQUEST
        }
    }
}
