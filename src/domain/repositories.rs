use async_trait::async_trait;
use crate::domain::entities::{Category, Product, User};
use crate::domain::values::categories::CategoryId;
use crate::domain::values::products::ProductName;
use crate::domain::values::users::UserName;
use crate::Result;

/// 商品を扱うリポジトリ
#[async_trait]
pub trait ProductRepository: Send + Sync + 'static {
    type Transaction;
    /// 商品キーワード検索する
    async fn select_by_name_like(&self , _: &Self::Transaction , keyword: &ProductName) -> Result<Vec<Product>>;
    /// 新しい商品を永続化する
    async fn insert(&self , _: &Self::Transaction , product: &Product) -> Result<Product>;
    /// 商品の存在確認
    async fn exists(&self , _:&Self::Transaction , name: &ProductName) -> Result<bool>;
}
/// カテゴリを扱うリポジトリ
#[async_trait]
pub trait CategoryRepository : Send + Sync + 'static {
    type Transaction;
    ///　すべてのカテゴリを取得する
    async fn select_all(&self , _: &Self::Transaction) -> Result<Vec<Category>>;
    ///　指定された識別子でカテゴリを取得する
    async fn select_by_id(&self , _: &Self::Transaction , id: &CategoryId) -> Result<Option<Category>>;
}
/// ユーザーを扱うリポジトリ
#[async_trait]
pub trait UserRepository : Send + Sync + 'static {
    type Transaction;
    /// 指定されたユーザー名で問合せする
    async fn select_by_name(&self , _: &Self::Transaction, user_name: &UserName) -> Result<Option<User>>;
    /// 新しいユーザーを永続化する
    async fn insert(&self , _: &Self::Transaction , user: &User) -> Result<User>;
}