use std::io::{Error, ErrorKind};
/// オリジナルのエラー
pub enum OriginalErrorKind {
    NonPasswordError,
    FoundSameNameError,
}

pub struct OriginalError {
    kind: OriginalErrorKind,
    pub error: sqlx::Error,
}
/// パスワード入力が必要な時に、入力されていなかった場合に出すエラー。
pub fn non_password_error() -> OriginalError {
    OriginalError {
        kind: OriginalErrorKind::NonPasswordError,
        error: sqlx::Error::Io(Error::new(ErrorKind::Other, "password is required.")),
    }
}

/// 同じusernameがdbにあった時に出すエラー。
pub fn found_same_name_error() -> OriginalError {
    OriginalError {
        kind: OriginalErrorKind::FoundSameNameError,
        error: sqlx::Error::Io(Error::new(ErrorKind::Other, "found same name user.")),
    }
}
