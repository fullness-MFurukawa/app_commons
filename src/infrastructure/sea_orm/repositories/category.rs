use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{DatabaseTransaction , EntityTrait};
use crate::{Result, AppError};
use crate::domain::entities::Category;
use crate::domain::repositories::CategoryRepository;
use crate::domain::values::categories::CategoryId;
use crate::domain::values::ValueInto;
use crate::infrastructure::converter::{ModelAndEntity, VecModelToVecEntity};
use crate::infrastructure::sea_orm::converter_impl::CategoryConverter;
use crate::infrastructure::sea_orm::models::prelude::SeaOrmProductCategory;


///
///  商品カテゴリ Repository
///
pub struct CategoryRepositoryImpl;
impl CategoryRepositoryImpl {
    //  Repositoryの生成
    pub fn new() -> Arc<dyn CategoryRepository<Transaction=DatabaseTransaction>> {
        Arc::new(Self{})
    }
}
#[async_trait]
impl CategoryRepository for CategoryRepositoryImpl{
    type Transaction = sea_orm::DatabaseTransaction;
    ///　すべてのカテゴリを取得する
    async fn select_all(&self, tran: &Self::Transaction) -> Result<Vec<Category>> {
        match SeaOrmProductCategory::find().all(tran).await{
            Ok(models) => CategoryConverter::entities(&models) ,
            Err(error) => Err(AppError::from(error))
        }
    }
    ///　指定された識別子でカテゴリを取得する
    async fn select_by_id(&self, tran: &Self::Transaction, id: &CategoryId) -> Result<Option<Category>> {
        match SeaOrmProductCategory::find_by_id(id.value()).one(tran).await{
            Ok(option_model) => {
                match option_model{
                    Some(model) => Ok(CategoryConverter::model_to_entity(&model).ok()) ,
                    None => Ok(None)
                }
            },
            Err(error) => Err(AppError::from(error))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::pool::PoolProvider;
    use crate::infrastructure::sea_orm::pool_impl::SeaOrmPool;
    use sea_orm::TransactionTrait;
    use super::*;

    #[actix::test]
    async fn select_all() -> Result<()> {
        let conn = SeaOrmPool::get().await;
        let tran = conn.begin().await.unwrap();

        let repository = CategoryRepositoryImpl::new();
        let categories = repository.select_all(&tran).await?;
        for category in categories {
            println!("{:?}", category);
        }
        Ok(())
    }

    #[actix::test]
    async fn select_by_id() -> Result<()> {
        let conn = SeaOrmPool::get().await;
        let tran = conn.begin().await.unwrap();

        let repository = CategoryRepositoryImpl::new();
        let category = repository.select_by_id(&tran , CategoryId::try_from(1)?).await.unwrap();
        println!("{:?}" , category);
        let err = repository.select_by_id(&tran , CategoryId::try_from(10)?).await.err().unwrap();
        println!("{:?}" , err);
        Ok(())
    }
}