use sqlx::postgres::PgPool;

use serde::{Deserialize, Serialize};

use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub userid: Option<i64>,
    pub username: String,
    pub password: Option<String>,
    pub mailaddress: Option<String>,
}

impl UserInfo {
    pub async fn check_user(&self, pool: &PgPool) -> bool {
        sqlx::query!(
            r##"SELECT id FROM public."users" WHERE name=$1"##,
            self.username,
        )
        .fetch_one(pool)
        .await
        .is_ok()
    }

    // nameとpasswordからユーザーを検索する。
    // UserInfoにパスワードが格納されていない時はfalse
    // 一つだけデータが取得出来たらtrue, そうでないときはfalseを返す。
    pub async fn find_user(&self, pool: &PgPool) -> bool {
        if self.password == None {
            println!("find_user にpassword がないデータが投入された");
            false
        } else {
            sqlx::query!(
                r##"SELECT id, mail_address, password FROM public."users" WHERE name=$1 and password=$2"##,
                self.username,
                self.password.as_ref().unwrap()
            )
            .fetch_one(pool)
            .await
            .is_ok()
        }
    }

    // nameに被りが無ければユーザー登録する。
    // passwordが無ければエラーを返す
    // ユーザー登録に成功すればtrue, 失敗すればfalse
    pub async fn signup_user(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        let non_password_error = Error::new(ErrorKind::Other, "password is required.");
        if self.password == None {
            return Err(sqlx::Error::Io(non_password_error));
        }

        let found_same_name_error = Error::new(ErrorKind::Other, "found same name user.");
        if self.check_user(pool).await {
            return Err(sqlx::Error::Io(found_same_name_error));
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
    // ユーザー登録情報を編集する。
    // usernameは変えれない。
    // 失敗した時はエラーを返す
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

    // username で登録車情報を探す
    // もしも登録されていなかったらエラーを返す。
    pub async fn get_user_info_from_name(&self, pool: &PgPool) -> Result<UserInfo, sqlx::Error> {
        let user = sqlx::query!(
            r##"SELECT id , name , password, mail_address FROM public."users" WHERE name=$1"##,
            self.username,
        )
        .fetch_one(pool)
        .await?;
        Ok(UserInfo {
            userid: Some(user.id),
            username: user.name,
            password: Some(user.password),
            mailaddress: user.mail_address, // DB上では、mail_address はNullable なのでOption<String>が返ってくる
        })
    }
}
