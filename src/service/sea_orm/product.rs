use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use crate::{AppError, Result};
use crate::domain::entities::Product;
use crate::domain::repositories::ProductRepository;
use crate::domain::services::ProductService;
use crate::domain::values::products::ProductName;
use crate::domain::values::ValueInto;
use crate::infrastructure::sea_orm::repositories::product::ProductRepositoryImpl;

///
/// 商品サービスの実装
///
pub struct ProductServiceImpl{
    // サービスで利用するリポジトリ
    repository: Arc<dyn ProductRepository<Transaction=DatabaseTransaction>>
}
impl ProductServiceImpl{
    // インスタンスをProductService型に変換して返す
    pub fn new() -> Arc<dyn ProductService<Database=DatabaseConnection>> {
        // Repositoryを生成してフィールドにセットする
        Arc::new(Self{ repository: ProductRepositoryImpl::new() })
    }
}
#[async_trait]
impl ProductService for ProductServiceImpl{
    type Database = DatabaseConnection;
    // 指定されたキーワードの商品を取得する
    async fn by_keyword(&self, db: &Self::Database, keyword: &ProductName) -> Result<Vec<Product>> {
        // トランザクションを開始する
        let tran = match db.begin().await {
            Ok(tran) => tran ,
            Err(error) => return Err(AppError::from(error)) // 内部エラーを返す
        };
        // Repositoryのメソッドを利用してキーワード検索する
        let products = self.repository.select_by_name_like(&tran, keyword).await?;
        if products.is_empty() {
            // 結果が空の場合、検索エラーメッセージを返す
            Err(AppError::SearchError(format!("キーワード:{} を含んだ商品は見つかりません。", keyword.value())))
        } else {
            Ok(products)  // 空でなければそのまま結果を返す
        }
    }
    // 商品を永続化する
    async fn register(&self, db: &Self::Database , product: &Product) -> Result<Product> {
        let tran = match db.begin().await {
            Ok(tran) => tran ,
            Err(error) => return Err(AppError::from(error))
        };
        // Repositoryを利用して商品を永続化する
        let new_product= self.repository.insert(&tran , product).await?;
        // トランザクションをコミットする
        match tran.commit().await{
            Ok(_) => Ok(new_product) ,
            Err(error) => Err(AppError::from(error))
        }
    }
    // 商品の存在確認する
    async fn exists(&self, db: &Self::Database , name: &ProductName) -> Result<()> {
        let tran = match db.begin().await {
            Ok(tran) => tran ,
            Err(error) => return Err(AppError::from(error))
        };
        let exists = self.repository.exists(&tran , name).await?;
        if exists {
            Err(AppError::RegisterError(format!("{}は登録済です。",name.value())))
        }else{
            Ok(())
        }
    }
}