use std::borrow::Borrow;
use std::sync::Arc;
use async_trait::async_trait;
use crate::{AppError, Result};
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use crate::domain::entities::User;
use crate::domain::repositories::UserRepository;
use crate::domain::services::UserService;
use crate::domain::values::ValueInto;
use crate::infrastructure::sea_orm::repositories::user::UserRepositoryImpl;

///
/// ユーザーサービスの実装
///
pub struct UserServiceImpl{
    repository: Arc<dyn UserRepository<Transaction=DatabaseTransaction>>
}
impl UserServiceImpl{
    pub fn new() -> Arc<dyn UserService<Database=DatabaseConnection>>{
        Arc::new(Self{ repository: UserRepositoryImpl::new() })
    }
}
#[async_trait]
impl UserService for UserServiceImpl{
    type Database = sea_orm::DatabaseConnection;
    async fn register(&self, db: &Self::Database , user: &User) -> Result<User> {
        let tran = match db.begin().await{
            Ok(tran) => tran ,
            Err(error) => return Err(AppError::from(error))
        };
        let new_user = self.repository.insert(&tran , user).await?;
        match tran.commit().await{
            Ok(_) => Ok(new_user),
            Err(error) => Err(AppError::from(error))
        }
    }

    async fn authenticate(&self, db: &Self::Database , user: &User) -> Result<User> {
        match db.begin().await{
            Ok(tran) =>{
                let opt_user = self.repository.select_by_name(&tran , user.user_name.borrow()).await?;
                match opt_user {
                    Some(get_user) => {
                        if user.password.value().eq(&get_user.password.value()){
                            Ok(get_user.clone())
                        }else{
                            Err(AppError::AuthenticateError(String::from("パスワードが異なります。")))
                        }
                    },
                    None => Err(AppError::AuthenticateError(String::from("存在しないユーザー名です。")))
                }
            } ,
            Err(error) => Err(AppError::from(error))
        }
    }
}