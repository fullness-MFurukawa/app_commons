use std::borrow::Borrow;
use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use crate::application::app_service::ProductRegisterAppService;
use crate::application::transfers::{CategoryDto, EntityToDto, ProductDto};
use crate::Result;
use crate::domain::entities::Characteristic;
use crate::domain::services::{CategoryService, ProductService};
use crate::domain::values::products::ProductName;
use crate::service::sea_orm::category::CategoryServiceImpl;
use crate::service::sea_orm::product::ProductServiceImpl;
use crate::presentation::forms::{FormToDomain, ProductRegisterForm};


///
/// 商品登録アプリケーションサービスの実装
///
pub struct ProductRegisterAppServiceImpl{
    // カテゴリサービス
    category_service: Arc<dyn CategoryService<Database=DatabaseConnection>> ,
    // 商品サービス
    product_service: Arc<dyn ProductService<Database=DatabaseConnection>>
}
impl ProductRegisterAppServiceImpl {
    pub fn new() -> Arc<dyn ProductRegisterAppService<
                                Pool=DatabaseConnection,Form=ProductRegisterForm>>{
        Arc::new(Self{
            category_service:CategoryServiceImpl::new() ,
            product_service:ProductServiceImpl::new()
        })
    }
}
#[async_trait]
impl ProductRegisterAppService for ProductRegisterAppServiceImpl {
    type Pool = DatabaseConnection;
    type Form = ProductRegisterForm;

    // 商品カテゴリを取得する
    async fn categories(&self, pool: &Self::Pool) -> Result<Vec<CategoryDto>> {
        let categories = self.category_service.all(pool).await?;
        // 取得結果をVec<CategoryDto>に変換して返す
        Ok(CategoryDto::converts(&categories))
    }
    // 新商品を登録する
    async fn execute(&self, pool: &Self::Pool, form: &Self::Form) -> Result<ProductDto> {
        // 商品の存在チェック
        let product_name = ProductName::try_from(form.name.as_ref().unwrap().clone())?;
        self.product_service.exists(pool , &product_name).await?;
        // 商品を登録する
        let mut product = self.product_service.register(pool , &form.convert()?).await?;
        // カテゴリを取得して 商品Entityのcategoryに格納する
        product.category = self.category_service.by_id(pool , product.category.unwrap().get().borrow()).await.ok();
        Ok(ProductDto::convert(&product))
    }
}