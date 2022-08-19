use std::{env, net::SocketAddr, time::Duration};

use axum::{
    extract,
    http::{HeaderValue, Method, StatusCode},
    response,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use dotenv::dotenv;

use sqlx::postgres::{PgPool, PgPoolOptions};

use tower_http::cors::CorsLayer;

use tracing_subscriber::fmt;

pub mod models;
use crate::models::{query::*, schemas::*};

pub mod users;
use crate::users::auth::*;
use crate::users::doc::*;

pub mod errors;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // db connection string
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //db connection pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("can connect to database");

    // tracing のフォーマット作成
    let format = fmt::format().with_level(true).with_target(true).compact();
    // initialize tracing
    tracing_subscriber::fmt().event_format(format).init();

    // build our application with a route
    let app = Router::new()
        .route("/get_hierarchy", post(get_hierarchy))
        .route("/page", post(register_page))
        .route("/doc", post(create_doc))
        .route("/app/:app_id/page", get(try_get_page))
        .route("/doc/:user_id", get(get_doc_infos))
        .route("/page/:hierarchy_id", get(get_page))
        .route("/login", post(login))
        .route("/user/:user_name", get(get_user))
        .route("/user", post(add_user))
        .route("/user", put(edit_user_info))
        .route("/page/:hierarchy_id", delete(delete_page))
        .layer(extract::Extension(pool)) // db poolを渡す
        // Corsの設定
        .layer(
            CorsLayer::new()
                .allow_origin([
                    env::var("CORS_ORIGIN")
                        .expect("CORS_ORIGIN must be set")
                        .parse::<HeaderValue>()
                        .unwrap(),
                    env::var("CORS_LOCAL_ORIGIN")
                        .expect("CORS_LOCAL_ORIGIN must be set")
                        .parse::<HeaderValue>()
                        .unwrap(),
                ])
                .allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT])
                .allow_headers(vec![http::header::CONTENT_TYPE]),
        );

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// ページパスを検索する為のクエリ
#[derive(Debug, Deserialize, Serialize)]
struct PageQuery {
    page_path: String,
}

// デフォルト値の設定
impl Default for PageQuery {
    fn default() -> Self {
        Self {
            page_path: String::from(""),
        }
    }
}

/// もしもデータが存在すればデータを返す。
/// なければNoneを返す
async fn try_get_page(
    extract::Path(app_id): extract::Path<i64>,
    page_path: Option<extract::Query<PageQuery>>,
    extract::Extension(pool): extract::Extension<PgPool>,
) -> impl IntoResponse {
    let extract::Query(page_path) = page_path.unwrap_or_default();
    let web_pages_or_none = is_exist_page(&pool, app_id, &page_path.page_path).await;
    if web_pages_or_none {
        StatusCode::OK //存在するのでcode 200を返す
    } else {
        StatusCode::NOT_FOUND // 存在しないのでcode 404 を返す
    }
}

/// web_pages, page_hierarchyにデータを作成する。
/// dbに既にデータが存在すれば、ファイルだけ更新する
async fn register_page(
    extract::Json(page_info): extract::Json<WebPageInfo>,
    extract::Extension(pool): extract::Extension<PgPool>,
) -> impl IntoResponse {
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
    extract::Extension(pool): extract::Extension<PgPool>,
) -> response::Json<Vec<HierarchyTS>> {
    if info.id == None {
        Json(get_page_structure(&pool, info.app_id).await)
    } else {
        Json(get_page_structure_from_id(&pool, info.id.unwrap()).await)
    }
}

///web_pages, page_hierarchy からデータを削除する。
///成功すればステータスコード202, 失敗すれば500を返す
async fn delete_page(
    extract::Path(hierarchy_id): extract::Path<i64>,
    extract::Extension(pool): extract::Extension<PgPool>,
) -> impl IntoResponse {
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
async fn get_page(
    extract::Path(hierarchy_id): extract::Path<i64>,
    extract::Extension(pool): extract::Extension<PgPool>,
) -> extract::Json<Page> {
    let (app_id, page_path) = get_web_page_info(&pool, hierarchy_id).await.unwrap();
    let web_page = get_web_page(&pool, app_id, &page_path).await.unwrap();
    let web_page_info = WebPageInfo {
        app_id,
        page_path: web_page.page_path,
        page_data: None,
    };

    // if let Ok(res) = fs::read_to_string(&web_page.file_path) {
    //     Json(Page {
    //         page_path: web_page.page_path,
    //         md: res,
    //     })
    // } else {
    //     tracing::error!("ファイルを読み込むのに失敗した");
    //     Json(Page {
    //         page_path: String::from(""),
    //         md: String::from(""),
    //     })
    // }

    match web_page_info.get_markdown().await {
        Ok(res) => Json(Page {
            page_path: web_page_info.page_path,
            md: res,
        }),
        Err(err) => {
            tracing::error!("{}", err.to_string());
            tracing::error!("ファイルを読み込むのに失敗した");
            Json(Page {
                page_path: String::from(""),
                md: String::from(""),
            })
        }
    }
}

/// id,pass に該当があればtrue,なければfalse
async fn login(
    extract::Json(info): extract::Json<LoginInfo>,
    extract::Extension(pool): extract::Extension<PgPool>,
) -> extract::Json<bool> {
    let can_login = info.can_login(&pool).await;
    Json(can_login)
}

/// dbにユーザーを登録する
/// 登録出来れば201, 出来なければ400を返す
async fn add_user(
    extract::Json(user): extract::Json<UserInfo>,
    extract::Extension(pool): extract::Extension<PgPool>,
) -> impl IntoResponse {
    if let Err(e) = user.signup_user(&pool).await {
        tracing::error!("In add_user error occured:{}", e);
        return StatusCode::BAD_REQUEST;
    };
    StatusCode::CREATED
}

/// ユーザー情報を編集する
async fn edit_user_info(
    extract::Json(user): extract::Json<UserInfo>,
    extract::Extension(pool): extract::Extension<PgPool>,
) -> impl IntoResponse {
    if let Err(e) = user.edit_user(&pool).await {
        tracing::error!("In edit_user_info error occured:{}", e);
        return StatusCode::BAD_REQUEST;
    };
    StatusCode::ACCEPTED
}

/// username からユーザー情報を検索する。
/// もしもエラーが起きたらdefaultのデータを返す
async fn get_user(
    extract::Path(user_name): extract::Path<String>,
    extract::Extension(pool): extract::Extension<PgPool>,
) -> extract::Json<UserInfo> {
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
async fn get_doc_infos(
    extract::Path(user_id): extract::Path<i64>,
    extract::Extension(pool): extract::Extension<PgPool>,
) -> extract::Json<Vec<DocInfo>> {
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
async fn create_doc(
    extract::Json(doc_info): extract::Json<DocInfo>,
    extract::Extension(pool): extract::Extension<PgPool>,
) -> impl IntoResponse {
    let res = doc_info.create_doc(&pool).await;
    match res {
        Ok(_) => StatusCode::CREATED,
        Err(e) => {
            tracing::error!("In create_doc error occured:{}", e);
            StatusCode::BAD_REQUEST
        }
    }
}
