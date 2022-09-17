use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use crate::app_commons::app_service::ProductSearchAppService;
use crate::app_commons::transfers::{EntityToDto, ProductDto};
use crate::Result;
use crate::domain::services::ProductService;
use crate::domain::values::products::ProductName;
use crate::service::sea_orm::product::ProductServiceImpl;
use crate::view_commons::forms::{FormToDomain, ProductSearchForm};


///
/// 商品検索サービスの実装
///
pub struct ProductSearchAppServiceImpl{
    service: Arc<dyn ProductService<Database=DatabaseConnection>>
}
impl  ProductSearchAppServiceImpl {
    pub fn new() -> Arc<dyn ProductSearchAppService<Pool=DatabaseConnection ,
                                                    Form=ProductSearchForm>>{
        Arc::new(Self{ service:ProductServiceImpl::new() })
    }
}
#[async_trait]
impl ProductSearchAppService for ProductSearchAppServiceImpl{
    type Pool = DatabaseConnection;
    type Form = ProductSearchForm;
    // キーワード検索
    async fn search(&self, pool: &Self::Pool, form: &Self::Form) -> Result<Vec<ProductDto>> {
        // キーワードをProductNameに変換する
        let keyword:ProductName = form.convert()?;
        // 検索を実行する
        match  self.service.by_keyword(pool , &keyword).await {
            Ok(results) => Ok(ProductDto::converts(&results)) ,
            Err(error) => Err(error)
        }
    }
}