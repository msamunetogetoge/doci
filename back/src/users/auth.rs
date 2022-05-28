use sqlx::postgres::PgPool;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub password: String,
    pub mailaddress: String,
}

// mailaddressとpasswordからユーザーを検索する。
// 一つだけデータが取得出来たらtrue, そうでないときはfalseを返す。
pub async fn find_user(pool: &PgPool, mail_address: String, password: String) -> bool {
    sqlx::query!(r##"SELECT id, mail_address, password FROM public."users" WHERE mail_address=$1 and password=$2"##, mail_address, password)
        .fetch_one(pool)
        .await.is_ok()
}

// mailaddressに被りが無ければユーザー登録する。
// ユーザー登録に成功すればtrue, 失敗すればfalse
pub async fn signup_user(pool: &PgPool, user: UserInfo) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    if let Err(e) = sqlx::query!(
        r##"
    INSERT INTO public."users" (name, password, mail_address) VALUES ($1, $2, $3) RETURNING id"##,
        user.name,
        user.password,
        user.mailaddress
    )
    .fetch_one(&mut tx)
    .await
    {
        tx.rollback().await?;
        return Err(e);
    }
    tx.commit().await?;
    Ok(())
}

// ユーザー登録情報を編集する。
// 失敗した時はエラーを返す
pub async fn edit_user(pool: &PgPool, user: UserInfo) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    if let Err(e) = sqlx::query!(
        r##"
    UPDATE public."users" SET name=$1, password=$2, mail_address=$3 WHERE mail_address = $3 RETURNING id"##,
        user.name,
        user.password,
        user.mailaddress
    )
    .fetch_one(&mut tx)
    .await
    {
        tx.rollback().await?;
        return Err(e);
    }
    tx.commit().await?;
    Ok(())
}
