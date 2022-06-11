use sqlx::postgres::PgPool;

use serde::{Deserialize, Serialize};

use chrono::prelude::*;

/// front側にデータを渡すときに使うストラクト
#[derive(Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct DocInfo {
    app_id: Option<i64>,
    app_name: String,
    created_by: i64,
    created_at: Option<DateTime<Local>>,
}

/// users.user_id が作成したdocumentを取得する
pub async fn get_created_doc(pool: &PgPool, user_id: i64) -> Result<Vec<DocInfo>, sqlx::Error> {
    let created_docs = sqlx::query_as::<_,DocInfo>(r##"
    SELECT id as app_id, name as app_name, created_by, created_at FROM public."applications" WHERE created_by=$1 "
    "##).bind(user_id).fetch_all(pool).await;
    match created_docs {
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
        Ok(docs) => Ok(docs),
    }
}

impl DocInfo {
    /// insert 文を発行する
    pub async fn create_doc(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;
        let created_docs = sqlx::query(
            r##"
    INSERT INTO  (name, created_by)  VALUES ($1,$2) RETURNING id"
    "##,
        )
        .bind(&self.app_name)
        .bind(&self.created_by)
        .fetch_one(&mut tx)
        .await;
        match created_docs {
            Err(e) => {
                tx.rollback().await?;
                println!("{}", e);
                Err(e)
            }
            Ok(docs) => {
                tx.commit().await?;
                Ok(())
            }
        }
    }
}
