use sea_orm::DbErr;
use thiserror::Error;
///
/// アプリケーション全体で利用するエラー型
///
#[derive(Debug , Error)]
pub enum AppError{
    #[error("{0}")]
    SearchError(String) ,  // 検索処理エラー
    #[error("{0}")]
    RegisterError(String) , // 登録処理エラー
    #[error("{0}")]
    AuthenticateError(String) ,// 認証エラー
    #[error(transparent)]
    InternalError(#[from] anyhow::Error) // 永続化層のエラー , ドメインルールエラー
}
// SeaOrmのエラーをラップした内部エラーを生成する
impl From<DbErr> for AppError{
    fn from(err: DbErr) -> Self {
        AppError::InternalError(anyhow::Error::new(err))
    }
}
// メッセージをラップした内部エラーを生成する
impl From<&str> for AppError{
    fn from(msg: &str) -> Self {
        AppError::InternalError(anyhow::Error::msg(msg.to_string()))
    }
}


