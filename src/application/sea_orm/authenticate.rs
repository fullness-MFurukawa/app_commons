use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use crate::app_commons::app_service::AuthenticateAppService;
use crate::app_commons::transfers::{EntityToDto, UserDto};
use crate::Result;
use crate::domain::services::UserService;
use crate::service::sea_orm::user::UserServiceImpl;
use crate::view_commons::forms::{FormToDomain, LoginForm};


///
/// ユーザー認証アプリケーションサービスの実装
///
pub struct AuthenticateAppServiceImpl{
    service: Arc<dyn UserService<Database=DatabaseConnection>>
}
impl AuthenticateAppServiceImpl{
    pub fn new() -> Arc<dyn AuthenticateAppService<Pool=DatabaseConnection , Form=LoginForm>>{
        Arc::new(Self{service:UserServiceImpl::new()})
    }
}
#[async_trait]
impl AuthenticateAppService for AuthenticateAppServiceImpl{
    type Pool = DatabaseConnection;
    type Form = LoginForm;

    async fn execute(&self, pool: &Self::Pool, form: &Self::Form) -> Result<UserDto> {
        let user = form.convert()?;
        let result = self.service.authenticate(pool , &user).await?;
        Ok(UserDto::convert(&result))
    }
}