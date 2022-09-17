use sea_orm::DbErr;
use thiserror::Error;

///
/// アプリケーション エラー
///
#[derive(Debug , Error)]
pub enum AppError{
    // 検索処理エラー
    #[error("{0}")]
    SearchError(String) ,
    // 登録処理エラー
    #[error("{0}")]
    RegisterError(String) ,
    // 認証エラー
    #[error("{0}")]
    AuthenticateError(String) ,
    // 永続化層のエラー , ドメインルールエラー
    #[error(transparent)]
    InternalError(#[from] anyhow::Error)
}
// SeaOrmのエラーをラップした内部エラーを生成する
impl From<DbErr> for AppError{
    fn from(err: DbErr) -> Self {
        AppError::InternalError(anyhow::Error::new(err))
    }
}
// エラーメッセージをラップした内部エラーを生成する
impl From<&str> for AppError{
    fn from(msg: &str) -> Self {
        AppError::InternalError(anyhow::Error::msg(msg.to_string()))
    }
}


