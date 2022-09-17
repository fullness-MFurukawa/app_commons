use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{ DatabaseTransaction, EntityTrait, QueryFilter, ColumnTrait, QueryOrder };
use crate::{AppError, Result};
use crate::domain::entities::{Characteristic, Product};
use crate::domain::repositories::ProductRepository;
use crate::domain::values::products::{ProductId, ProductName};
use crate::domain::values::ValueInto;
use crate::infrastructure::converter::{ActiveModelGenerator, VecModelToVecEntity};
use crate::infrastructure::sea_orm::converter_impl::ProductConverter;
use crate::infrastructure::sea_orm::models::product;
use crate::infrastructure::sea_orm::models::prelude::SeaOrmProduct;
use crate::infrastructure::sea_orm::models::prelude::SeaOrmProductCategory;

///
/// 商品リポジトリの実装
///
pub struct ProductRepositoryImpl;
impl ProductRepositoryImpl{
    // インスタンスをProductRepository型に変換して返す
    pub fn new() -> Arc<dyn ProductRepository<Transaction=DatabaseTransaction>> {
        // インスタンスをスレッドセーフな参照カウンタArcにラップして返す
        Arc::new(Self{})
    }
}
#[async_trait]
impl ProductRepository for ProductRepositoryImpl{
    type Transaction = sea_orm::DatabaseTransaction;
    /// キーワード検索
    async fn select_by_name_like(&self, tran: &Self::Transaction, keyword: &ProductName) -> Result<Vec<Product>> {
        // 指定されたキーワードで問合せし、商品番号でソートした結果を取得する
        match SeaOrmProduct::find().filter(product::Column::Name.contains(keyword.value().as_str()))
            .find_also_related(SeaOrmProductCategory)
            .order_by_asc(product::Column::Id)
            .all(tran).await{
            Ok(models) => // 結合で取得したモデルをEntityに変換して返す
                ProductConverter::join_model_to_entities(&models) ,
            Err(error) => // SeaOrmからのエラーをAppErrorにラップして返す
                Err(AppError::from(error))
        }
    }
    /// 新商品の追加
    async fn insert(&self, tran: &Self::Transaction, product: &Product) -> Result<Product> {
        // 渡されたEntityをModelに変換する
        let new_product = ProductConverter::active_model(product);
        // データを永続化する
        match SeaOrmProduct::insert(new_product).exec(tran).await{
            Ok(new_id) => {
                // Entityのクローンを取得する
                let mut new_product = product.clone();
                // 返されたIdをProductIdに格納する
                let product_id = ProductId::try_from(new_id.last_insert_id as i32)?;
                // ProductIdを変更する
                new_product.change(&product_id)?;
                // 永続化結果を返す
                Ok(new_product)
            },
            Err(error) => Err(AppError::from(error))
        }
    }
    /// 商品の存在チェック
    async fn exists(&self, tran: &Self::Transaction, name: &ProductName) -> Result<bool> {
        match SeaOrmProduct::find()
            .filter(product::Column::Name.eq(name.value())).one(tran).await{
            Ok(result) => Ok(result.is_some()) ,
            Err(error) => Err(AppError::from(error))
        }
    }
}
#[cfg(test)]
mod tests{
    use crate::infrastructure::pool::PoolProvider;
    use crate::infrastructure::sea_orm::pool_impl::SeaOrmPool;
    use sea_orm::TransactionTrait;
    use super::*;

    #[actix::test]
    async fn select_by_name() -> Result<()> {
        let conn = SeaOrmPool::get().await;
        let tran = conn.begin().await.unwrap();
        let repository = ProductRepositoryImpl::new();
        let products = repository.select_by_name_like(
            &tran , ProductName::try_from(String::from("マウス"))?).await?;
        for product in products{
            println!("{:?}" , product);
        }
        let products = repository.select_by_name_like(
            &tran , ProductName::try_from(String::from("xxxx"))?).await?;
        if products.is_empty(){
            println!("Empty!!");
        }else{
            println!("Not Empty!!");
            println!("{:?}" , products);
        }
        Ok(())
    }
    #[actix::test]
    async fn exists() -> Result<()>{
        let conn = SeaOrmPool::get().await;
        let tran = conn.begin().await.unwrap();
        let repository = ProductRepositoryImpl::new();
        let result  = repository.exists(&tran ,
                                        ProductName::try_from(String::from("水性ボールペン(黒)"))?).await.unwrap();
        println!("result = {:?}" , result);
        let result  = repository.exists(&tran ,
                                        ProductName::try_from(String::from("水性ボールペン"))?).await.unwrap();
        println!("result = {:?}" , result);
        Ok(())
    }
}