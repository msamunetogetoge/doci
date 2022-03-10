use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use std::env;

/// users table
#[derive(sqlx::FromRow)]
pub struct Users {
    pub id: i64,
    pub name: String,
    pub password: String,
    pub mailaddress: String,
}

/// web_pages table
#[derive(sqlx::FromRow)]
pub struct WebPages {
    pub app_id: i64,
    pub page_path: String,
    pub file_path: String,
}

// post from client, document infomation
#[derive(Serialize, Deserialize, Clone)]
pub struct WebPageInfo {
    pub app_id: i64,
    pub page_path: String,
}

// .envファイルの情報からdbに接続する
pub async fn get_conn() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.unwrap()
}

// ドキュメントを取得する
pub async fn get_web_page(
    pool: &PgPool,
    id: i64,
    page_path: &str,
) -> Result<WebPages, sqlx::Error> {
    let page = sqlx::query_as::<_, WebPages>(r##"SELECT app_id, page_path, file_path FROM public."web_pages" WHERE app_id=$1 and page_path=$2"##)
        .bind(id)
        .bind(page_path)
        .fetch_one(pool)
        .await?;

    Ok(page)
}

// ドキュメントを追加する。 page_id はSerial で勝手に振られるので、適当な値を入れておく。
pub async fn add_web_page(pool: &PgPool, page: WebPageInfo) -> Result<(), sqlx::Error> {
    if get_web_page(pool, page.app_id, &page.page_path)
        .await
        .is_ok()
    {
        return Err(sqlx::Error::RowNotFound);
    }

    let file_path = create_file_path(&page);

    let page = sqlx::query!(
        r##" INSERT INTO public."web_pages" (app_id, page_path, file_path) VALUES ($1, $2, $3)"##,
        page.app_id,
        &page.page_path,
        file_path
    )
    .execute(pool)
    .await;
    match page {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

// アプリとパスの情報からmdの格納先を作成する
// 未作成
pub fn create_file_path(page: &WebPageInfo) -> &str {
    &page.page_path
}
