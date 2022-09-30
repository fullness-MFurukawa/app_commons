use async_trait::async_trait;
use crate::Result;
use crate::application::transfers::{CategoryDto, ProductDto, UserDto};
///
/// 商品検索アプリケーションサービス
///
#[async_trait]
pub trait ProductSearchAppService : Send + Sync + 'static {
    type Pool;
    type Form;
    // 検索処理
    async fn search(&self , pool: &Self::Pool , form: &Self::Form) -> Result<Vec<ProductDto>>;
}
///
/// 商品登録アプリケーションサービス
///
#[async_trait]
pub trait ProductRegisterAppService: Send + Sync + 'static {
    type Pool;
    type Form;
    // カテゴリリストの取得
    async fn categories(&self , pool:&Self::Pool) -> Result<Vec<CategoryDto>>;
    // 商品の登録
    async fn execute(&self , pool:&Self::Pool , form: &Self::Form) -> Result<ProductDto>;
}
///
/// 認証アプリケーションサービス
///
#[async_trait]
pub trait AuthenticateAppService: Send + Sync + 'static {
    type Pool;
    type Form;
    // ユーザーの認証
    async fn execute(&self , pool:&Self::Pool , form: &Self::Form) -> Result<UserDto>;
}
///
/// ユーザー登録アプリケーションサービス
///
#[async_trait]
pub trait UserRegisterAppService: Send + Sync + 'static {
    type Pool;
    type Form;
    // ユーザーの登録
    async fn execute(&self , pool:&Self::Pool , form: &Self::Form) -> Result<()>;
}
