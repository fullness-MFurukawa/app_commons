use async_trait::async_trait;
use crate::domain::entities::{Category, Product, User};
use crate::domain::values::categories::CategoryId;
use crate::domain::values::products::ProductName;
use crate::Result;
/// カテゴリを扱うService
#[async_trait]
pub trait CategoryService : Send + Sync + 'static {
    type Database;
    /// すべてのカテゴリを取得する
    async fn all(&self , _: &Self::Database) -> Result<Vec<Category>>;
    /// 指定されたカテゴリIdのカテゴリを取得する
    async fn by_id(&self , _: &Self::Database , id: &CategoryId) -> Result<Category>;
}
/// 商品を扱うService
#[async_trait]
pub trait ProductService : Send + Sync + 'static  {
    type Database;
    // 指定されたキーワードの商品を取得する
    async fn by_keyword(&self , _: &Self::Database , keyword: &ProductName) -> Result<Vec<Product>>;
    // 商品を永続化する
    async fn register(&self , _: &Self::Database, product: &Product) -> Result<Product>;
    // 商品の存在確認する
    async fn exists(&self , _: &Self::Database , name: &ProductName) -> Result<()>;
}
/// ユーザーを扱うService
#[async_trait]
pub trait UserService : Send + Sync + 'static {
    type Database;
    /// ユーザーを永続化する
    async fn register(&self , _: &Self::Database , user: &User) -> Result<User>;
    /// ユーザーを認証する
    async fn authenticate(&self , _:&Self::Database , user: &User) -> Result<User>;
}




