use std::sync::Arc;
use sea_orm::DatabaseConnection;
use crate::application::app_service::{AuthenticateAppService, ProductRegisterAppService, ProductSearchAppService};
use crate::application::sea_orm::authenticate::AuthenticateAppServiceImpl;
use crate::application::sea_orm::product_register::ProductRegisterAppServiceImpl;
use crate::application::sea_orm::product_search::ProductSearchAppServiceImpl;
use crate::presentation::forms::{LoginForm, ProductRegisterForm, ProductSearchForm};

///
/// アプリケーションサービスプロバイダ
///
#[derive(Clone)]
pub struct AppServiceProvider {
    // 商品検索サービス
    pub search_service: Arc<dyn ProductSearchAppService<Pool=DatabaseConnection,Form=ProductSearchForm>> ,
    // 商品登録ービス
    pub register_service: Arc<dyn ProductRegisterAppService<Pool=DatabaseConnection,Form=ProductRegisterForm>> ,
    // ユーザー認証サービス
    pub authenticate_service: Arc<dyn AuthenticateAppService<Pool=DatabaseConnection,Form=LoginForm>>
}
impl AppServiceProvider {
    pub fn new() -> Arc<Self> {
        Arc::new(
            Self{
                search_service:ProductSearchAppServiceImpl::new() ,
                register_service:ProductRegisterAppServiceImpl::new() ,
                authenticate_service:AuthenticateAppServiceImpl::new()
            })
    }
}