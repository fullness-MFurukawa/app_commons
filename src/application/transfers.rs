use serde::{Serialize, Deserialize};
use rusty_money::{iso, Money};
use crate::domain::entities::{Category, Characteristic, Product, User};
use crate::domain::values::ValueInto;


// EntityからDTOへの変換トレイト
pub trait EntityToDto<T>{
    // 1つのEntityからDTOに変換する
    fn convert(value:&T) -> Self;
    // 複数のEntityから複数のDTOに変換する
    fn converts(values: &Vec<T>) -> Vec<Self> where Self:Sized;
}


///
///　商品カテゴリDTO
///
#[derive( Serialize , Deserialize , Debug , Clone )]
pub struct CategoryDto {
    pub id:     String ,
    pub name:   String ,
}
// EntityからDTOに変換
impl EntityToDto<Category> for CategoryDto {
    fn convert(value: &Category) -> Self {
        Self{
            id: value.get().value().to_string() ,
            name: value.name.value()
        }
    }
    fn converts(values: &Vec<Category>) -> Vec<Self> where Self: Sized {
        let mut results:Vec<Self> = Vec::new();
        for value in values {
            results.push(Self::convert(value));
        }
        results
    }
}

///
/// 商品DTO
///
#[derive(Serialize , Deserialize , Debug , Clone)]
pub struct ProductDto {
    pub id:     String ,
    pub name:   String ,
    pub price:  String ,
    pub category: CategoryDto
}
// EntityからDTOに変換
impl EntityToDto<Product> for ProductDto {
    fn convert(value: &Product) -> Self {
        let _category = CategoryDto::convert(value.category.as_ref().unwrap());
        Self{
            id: value.get().value().to_string() ,
            name: value.name.value() ,
            // 通貨形式にフォーマット変換
            price: Money::from_minor(value.price.value() as i64, iso::JPY).to_string() ,
            category: _category ,
        }
    }
    fn converts(values: &Vec<Product>) -> Vec<Self> where Self: Sized {
        let mut results:Vec<Self> = Vec::new();
        for value in values {
            results.push(Self::convert(value));
        }
        results
    }
}
///
/// ユーザーDTO
///
#[derive(Serialize , Deserialize , Debug , Clone)]
pub struct UserDto{
    pub user_id:    String ,
    pub user_name:  String ,
    pub password:   String ,
    pub mail:       String
}
// EntityからDTOに変換
impl EntityToDto<User> for UserDto {
    fn convert(value: &User) -> Self {
        Self{
            user_id: value.get().value() ,
            user_name: value.user_name.value() ,
            password: value.password.value() ,
            mail: value.mail.value()
        }
    }
    fn converts(values: &Vec<User>) -> Vec<Self> where Self: Sized {
        let mut results:Vec<Self> = Vec::new();
        for value in values {
            results.push(Self::convert(value));
        }
        results
    }
}
