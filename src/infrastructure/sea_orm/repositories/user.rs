use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{DatabaseTransaction , EntityTrait , ColumnTrait , QueryFilter};
use crate::{AppError, Result};
use crate::domain::entities::User;
use crate::domain::repositories::UserRepository;
use crate::domain::values::users::UserName;
use crate::domain::values::ValueInto;
use crate::infrastructure::converter::{ActiveModelGenerator, ModelAndEntity};
use crate::infrastructure::sea_orm::converter_impl::UserConverter;
use crate::infrastructure::sea_orm::models::prelude::SeaOrmUser;
use crate::infrastructure::sea_orm::models::user;

///
/// ユーザーリポジトリの実装
///
pub struct UserRepositoryImpl;
impl UserRepositoryImpl{
    pub fn new() -> Arc<dyn UserRepository<Transaction=DatabaseTransaction>> {
        Arc::new(Self{})
    }
}
#[async_trait]
impl UserRepository for UserRepositoryImpl{
    type Transaction = sea_orm::DatabaseTransaction;
    /// 指定されたユーザー名で問合せする
    async fn select_by_name(&self, tran: &Self::Transaction, user_name: &UserName) -> Result<Option<User>> {
        match SeaOrmUser::find().filter(user::Column::UserName.eq(user_name.value().as_str())).one(tran).await {
            Ok(option_model) => {
                match option_model {
                    Some(model) => Ok(UserConverter::model_to_entity(&model).ok()),
                    None => Ok(None)
                }
            },
            Err(error) => Err(AppError::from(error))
        }
    }
    /// 新しいユーザーを永続化する
    async fn insert(&self, tran: &Self::Transaction, user: &User) -> Result<User> {
        let new_user = UserConverter::active_model(user);
        match SeaOrmUser::insert(new_user).exec(tran).await{
            Ok(_) => Ok(user.clone()),
            Err(error) => Err(AppError::from(error))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use sea_orm::TransactionTrait;
    use crate::domain::values::users::{Mail, Password, UserName};
    use crate::infrastructure::pool::PoolProvider;
    use crate::infrastructure::sea_orm::pool_impl::SeaOrmPool;

    #[actix::test]
    async fn select_by_user_name() -> Result<()> {
        let conn = SeaOrmPool::get().await;
        let tran = conn.begin().await.unwrap();
        let repository = UserRepositoryImpl::new();
        let user = repository.select_by_name(&tran,
        UserName::try_from(String::from("user001"))?).await?;
        println!("{:?}", user);
        let user = repository.select_by_name(&tran,
        UserName::try_from(String::from("abcd"))?).await?;
        println!("{:?}", user);

        Ok(())
    }

    #[actix::test]
    async fn insert() -> Result<()> {
        let password = "j2hcn6sU".to_string();
        let user = User::new(
            UserName::try_from("user002".to_string()) ? ,
            Password::try_from(password) ? ,
            Mail::try_from("user001@sample.com".to_string()) ?).unwrap();

        let conn = SeaOrmPool::get().await;
        let tran = conn.begin().await.unwrap();
        let repository = UserRepositoryImpl::new();
        let user = repository.insert(&tran , &user).await?;
        println!("{:?}" ,user);
        tran.rollback().await?;
        Ok(())
    }
}