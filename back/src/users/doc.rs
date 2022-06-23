use sqlx::postgres::PgPool;

use serde::{Deserialize, Serialize};

use crate::{HIERARCHY_TOP_NUMBER, HIERARCHY_TOP_PARENT_ID};
use chrono::NaiveDateTime;

/// front側にデータを渡すときに使うストラクト
#[derive(Serialize, Deserialize, sqlx::FromRow, Default)]
pub struct DocInfo {
    app_id: Option<i64>,
    app_name: String,
    created_by: i64,
    created_at: Option<NaiveDateTime>,
}

/// users.user_id が作成したdocumentを取得する
pub async fn get_created_doc(pool: &PgPool, user_id: i64) -> Result<Vec<DocInfo>, sqlx::Error> {
    let created_docs = sqlx::query_as::<_,DocInfo>(r##"
    SELECT id as app_id, name as app_name, created_by, created_at FROM public."applications" WHERE created_by=$1
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
    /// insert 文を発行すし、page_hierarchy にもデータを作成する。
    pub async fn create_doc(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        // struct で定義する

        let mut tx = pool.begin().await?;
        // query_as! するために使う
        struct ID {
            id: i64,
        }
        let created_doc = sqlx::query_as!(
            ID,
            r##"
    INSERT INTO  public."applications" (name, created_by)  VALUES ($1,$2) RETURNING id
    "##,
            &self.app_name,
            &self.created_by,
        )
        .fetch_one(&mut tx)
        .await;
        // insert 成功ならpage_hierarchy にも初期値データを格納する
        match created_doc {
            Err(e) => {
                tx.rollback().await?;
                println!("{}", e);
                Err(e)
            }
            Ok(row) => {
                let created_hierarchy = sqlx::query(
                    r##"
            INSERT INTO  public."page_hierarchy" (app_id, parent, child, depth)  VALUES ($1,$2,$3,$4) RETURNING id
            "##,
                )
                .bind(row.id)
                .bind(HIERARCHY_TOP_PARENT_ID)
                .bind(&self.app_name)
                .bind(HIERARCHY_TOP_NUMBER)
                .fetch_one(&mut tx)
                .await;
                // insert 成功ならOKを返す
                match created_hierarchy {
                    Err(e) => {
                        tx.rollback().await?;
                        println!("{}", e);
                        Err(e)
                    }
                    Ok(_) => {
                        tx.commit().await?;
                        Ok(())
                    }
                }
            }
        }
    }
}
