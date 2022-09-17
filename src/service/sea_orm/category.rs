use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use crate::{AppError, Result};
use crate::domain::entities::Category;
use crate::domain::repositories::CategoryRepository;
use crate::domain::services::CategoryService;
use crate::domain::values::categories::CategoryId;
use crate::domain::values::ValueInto;
use crate::infrastructure::sea_orm::repositories::category::CategoryRepositoryImpl;

///
/// カテゴリサービスの実装
///
pub struct CategoryServiceImpl{
    repository: Arc<dyn CategoryRepository<Transaction=DatabaseTransaction>>
}
impl CategoryServiceImpl{
    pub fn new() -> Arc<dyn CategoryService<Database=DatabaseConnection>> {
        Arc::new(Self{ repository: CategoryRepositoryImpl::new() })
    }
}
#[async_trait]
impl CategoryService for CategoryServiceImpl{
    type Database = sea_orm::DatabaseConnection;
    async fn all(&self, db: &Self::Database) -> Result<Vec<Category>> {
        match db.begin().await {
            Ok(tran) => Ok(self.repository.select_all(&tran).await?) ,
            Err(error) => Err(AppError::from(error))
        }
    }

    async fn by_id(&self, db: &Self::Database , id: &CategoryId) -> Result<Category> {
        match db.begin().await {
            Ok(tran) => {
                let category = self.repository.select_by_id(&tran, id).await?;
                if category.is_some() {
                    Ok(category.unwrap())
                } else {
                    Err(AppError::SearchError(format!("カテゴリ番号{}に該当データがありません。", id.value())))
                }
            },
            Err(error) => Err(AppError::from(error))
        }
    }
}