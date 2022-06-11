use sqlx::postgres::PgPool;

use serde::{Deserialize, Serialize};

use crate::errors::original_error;

/// front側と通信する時に使うストラクト
/// ユーザーデータを格納する
#[derive(Serialize, Deserialize, Default)]
pub struct UserInfo {
    /// users.id
    pub userid: Option<i64>,
    /// users.username
    pub username: String,
    /// users.password
    pub password: Option<String>,
    /// users.mail_address
    pub mailaddress: Option<String>,
}

impl UserInfo {
    pub fn check_password(&self) -> bool {
        self.password == None || self.password == Some("".to_string())
    }

    /// すでにユーザーデータが存在すればtrue
    pub async fn check_user(&self, pool: &PgPool) -> bool {
        sqlx::query!(
            r##"SELECT id FROM public."users" WHERE name=$1"##,
            self.username,
        )
        .fetch_one(pool)
        .await
        .is_ok()
    }

    /// usernameに被りが無ければユーザー登録する。
    /// passwordが無ければエラーを返す
    /// ユーザー登録に成功すればOk(()), 失敗すればerror
    pub async fn signup_user(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        if self.check_password() {
            return Err(original_error::non_password_error().error);
        }
        if self.check_user(pool).await {
            return Err(original_error::found_same_name_error().error);
        }
        // transaction start
        let mut tx = pool.begin().await?;

        if let Err(e) = sqlx::query!(
            r##"
    INSERT INTO public."users" (name, password, mail_address) VALUES ($1, $2, $3) RETURNING id"##,
            self.username,
            self.password,
            self.mailaddress
        )
        .fetch_one(&mut tx)
        .await
        {
            tx.rollback().await?;
            return Err(e);
        }
        tx.commit().await?;
        Ok(())
        // transaction end
    }
    /// ユーザー登録情報を編集する。
    /// usernameは変えれない。
    /// 失敗した時はエラーを返す
    pub async fn edit_user(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;
        if let Err(e) = sqlx::query!(
            r##"
    UPDATE public."users" SET password=$2, mail_address=$3 WHERE name = $1 RETURNING id"##,
            self.username,
            self.password,
            self.mailaddress
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

    /// username で登録車情報を探す
    /// もしも登録されていなかったらエラーを返す。
    pub async fn get_user_info_from_name(&self, pool: &PgPool) -> Result<UserInfo, sqlx::Error> {
        let user = sqlx::query!(
            r##"SELECT id , name , mail_address FROM public."users" WHERE name=$1"##,
            self.username,
        )
        .fetch_one(pool)
        .await?;
        Ok(UserInfo {
            userid: Some(user.id),
            username: user.name,
            password: None,
            mailaddress: user.mail_address, // DB上では、mail_address はNullable なのでOption<String>が返ってくる
        })
    }
}

/// loginに使うstruct
#[derive(Serialize, Deserialize, Debug)]

pub struct LoginInfo {
    /// users.username
    username: String,
    /// users.password
    password: String,
}

impl LoginInfo {
    /// nameとpasswordからユーザーを検索する。
    /// 一つだけデータが取得出来たらtrue, そうでないときはfalseを返す。
    pub async fn can_login(&self, pool: &PgPool) -> bool {
        sqlx::query!(
            r##"SELECT id FROM public."users" WHERE name=$1 and password=$2"##,
            self.username,
            self.password
        )
        .fetch_one(pool)
        .await
        .is_ok()
    }
}
