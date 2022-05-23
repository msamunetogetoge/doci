use dotenv::dotenv;
// use serde::{de::value::StrDeserializer, Deserialize, Serialize};
use sqlx::{postgres::PgPool, Row};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

// mailaddressとpasswordからユーザーを検索する。
// 一つだけデータが取得出来たらtrue, そうでないときはfalseを返す。
pub async fn find_user(pool: &PgPool, mail_address: String, password: String) -> bool {
    sqlx::query!(r##"SELECT id, mail_address, password FROM public."users" WHERE mail_address=$1 and password=$2"##, mail_address, password)
        .fetch_one(pool)
        .await.is_ok()
}
