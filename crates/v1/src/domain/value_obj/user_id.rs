use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserId(i64);

impl UserId {
    /// user_idが正の整数であることを保証する。
    /// - user_idは本システムにおいて必須の値の為，存在は保証されている。
    pub fn new(user_id: i64) -> AppResult<Self> {
        if user_id <= 0 {
            return Err(AppError::InternalServerError(Some(
                "UserIdは正の整数でなければなりません。".into(),
            )));
        }
        Ok(Self(user_id))
    }

    /// user_idの実態(i64)を返す。
    pub fn as_i64(self) -> i64 {
        self.0
    }
}
