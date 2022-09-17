use sea_orm::ActiveValue::{NotSet, Set};
use crate::Result;
use crate::domain::entities::{Category, Product, User};
use crate::domain::values::categories::{CategoryId, CategoryName};
use crate::domain::values::products::{ProductId, ProductName, ProductPrice};
use crate::domain::values::users::{Mail, Password, UserId, UserName};
use crate::domain::values::ValueInto;
use crate::domain::entities::Characteristic;
use crate::infrastructure::sea_orm::models::product_category;
use crate::infrastructure::sea_orm::models::product;
use crate::infrastructure::sea_orm::models::user;
use crate::infrastructure::converter::{ActiveModelGenerator, ModelAndEntity, VecModelToVecEntity};

pub struct CategoryConverter;
impl ModelAndEntity for CategoryConverter{
    type Entity = Category;
    type Model = product_category::Model;

    fn model_to_entity(model: &Self::Model) -> Result<Self::Entity> {
        let m = model.clone();
        Ok(Category::new(
            CategoryId::try_from(m.id)? ,
            CategoryName::try_from(m.name.unwrap())?))
    }

    fn entity_to_model(entity: &Self::Entity) -> Self::Model {
        Self::Model{
            id: entity.get().value() ,
            name: Some(entity.name.value())
        }
    }
}
impl VecModelToVecEntity for CategoryConverter {
    type Entity = Category;
    type Model = product_category::Model;
    type JoinModel = ();
    fn entities(models: &Vec<Self::Model>) -> Result<Vec<Self::Entity>>{
        let mut categories:Vec<Self::Entity> = Vec::new();
        for model in models {
            let m = model.clone();
            let category = Category::new(
                CategoryId::try_from(m.id)? ,
                CategoryName::try_from(m.name.unwrap())?);
            categories.push(category);
        }
        Ok(categories)
    }
}

///
/// 商品情報の変換
///
pub struct ProductConverter;
// ORMモデルとEntityの相互変換
impl ModelAndEntity for ProductConverter{
    type Entity = Product; // 商品Entity
    type Model = product::Model; // 商品ORMモデル
    // ORMモデルからEntityに変換する
    fn model_to_entity(model: &Self::Model) -> Result<Self::Entity> {
        let m = model.clone();
        // カテゴリEntityを生成する
        let category = Category::new(
            CategoryId::try_from(m.category_id.unwrap())? ,
            CategoryName::try_from(String::from("dummy"))?);
        // 商品Entityを生成して返す
        Ok(Product::new(
            ProductId::try_from(m.id)? ,
            ProductName::try_from(m.name.unwrap())? ,
            ProductPrice::try_from(m.price.unwrap())? ,
            Some(category)))
    }
    // EntityをORMモデルに変換する
    fn entity_to_model(entity: &Self::Entity) -> Self::Model {
        Self::Model {
            id: entity.get().value() ,
            name: Some(entity.name.value()) ,
            price: Some(entity.price.value()) ,
            category_id: Some(entity.category.as_ref().unwrap().get().value())
        }
    }
}
impl VecModelToVecEntity for ProductConverter {
    type Entity = Product;
    type Model = product::Model;
    type JoinModel = product_category::Model;
    // 結合結果のORM ModelをEntityに変換する
    fn join_model_to_entities(models: &Vec<(Self::Model , Option<Self::JoinModel>)>) -> Result<Vec<Self::Entity>>{
        let mut products:Vec<Product> = Vec::new();
        for model in models{
            let m = model.clone();
            let category = Category::new(
                CategoryId::try_from(m.1.as_ref().unwrap().id)? ,
                CategoryName::try_from(m.1.unwrap().name.unwrap())?);
            let product = Product::new(
                ProductId::try_from(m.0.id)? ,
                ProductName::try_from(m.0.name.unwrap())?,
                ProductPrice::try_from(m.0.price.unwrap())?,
                Some(category));
            products.push(product);
        }
        Ok(products)
    }
}
// EntityをActiveModelに変換する
impl ActiveModelGenerator for ProductConverter {
    type Entity = Product;
    type ActiveModel = product::ActiveModel;
    fn active_model(entity: &Self::Entity) -> Self::ActiveModel {
        Self::ActiveModel {
            id: NotSet,
            name: Set(Some(entity.name.value())),
            price: Set(Some(entity.price.value())),
            category_id: Set(Some(entity.category.as_ref().unwrap().get().value()))
        }
    }
}

pub struct UserConverter;
impl ModelAndEntity for UserConverter {
    type Entity = User;
    type Model = user::Model;

    fn model_to_entity(model: &Self::Model) -> Result<Self::Entity> {
        let m = model.clone();
        Ok(User::rebuilding(
            UserId::try_from(m.user_id.unwrap())?,
            UserName::try_from(m.user_name.unwrap())?,
            Password::try_from(m.password.unwrap())?,
            Mail::try_from(m.mail.unwrap())?))
    }
    fn entity_to_model(entity: &Self::Entity) -> Self::Model {
        Self::Model{
            id: 0 ,
            user_id: Some(entity.get().value()) ,
            user_name: Some(entity.user_name.value()) ,
            password: Some(entity.password.value()) ,
            mail: Some(entity.mail.value())
        }
    }
}
impl ActiveModelGenerator for UserConverter{
    type Entity = User;
    type ActiveModel = user::ActiveModel;
    fn active_model(entity: &Self::Entity) -> Self::ActiveModel {
        Self::ActiveModel{
            id: NotSet ,
            user_id: Set(Some(entity.get().value())) ,
            user_name: Set(Some(entity.user_name.value())) ,
            password: Set(Some(entity.password.value())) ,
            mail: Set(Some(entity.mail.value()))
        }
    }
}