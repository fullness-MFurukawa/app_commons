use std::collections::HashMap;
use anyhow::Result;
use serde::{de, Deserialize, Serialize};
use validator::{validate_length , validate_required , validate_range , Validate};
use crate::domain::entities::{Category, Product, User};
use crate::domain::values::categories::{CategoryId, CategoryName};
use crate::domain::values::products::{ProductId, ProductName, ProductPrice};
use crate::domain::values::users::{Mail, Password, UserName};
use crate::error::AppError;
use crate::presentation::validate::{AppValidator, ValidationError};


///
/// FormをDomain,Valueオブジェクトに変換する
///
pub trait FormToDomain<T>{
    fn convert(&self) -> Result<T , AppError>;
}

///
/// 空の文字列をNoneにマッピングする
///
pub fn empty_string_as_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: de::Deserialize<'de>, D: de::Deserializer<'de> {
    Ok(T::deserialize(deserializer).ok())
}

// 商品検索
#[derive(Deserialize , Debug)]
pub struct ProductSearchForm {
    pub keyword: Option<String>
}
/// 入力値検証
impl AppValidator for ProductSearchForm{
    fn validate_value(&self) -> Result<(), ValidationError> {
        let mut errors:HashMap<String,String> = HashMap::new();
        // 未入力と範囲チェック
        if self.keyword.is_none() || self.keyword.as_ref().unwrap().is_empty() {
            errors.insert(String::from("keyword"),String::from("キーワードは入力必須です。"));
        }
        if errors.is_empty(){
            Ok(())
        }else{
            Err(ValidationError::from(errors))
        }
    }
}
// FormをProductNameに変換する
impl FormToDomain<ProductName> for ProductSearchForm{
    fn convert(&self) -> anyhow::Result<ProductName, AppError> {
        Ok(ProductName::try_from(self.keyword.as_ref().unwrap().clone())?)
    }
}
// 商品登録
#[derive(Deserialize , Serialize , Debug , Clone)]
pub struct ProductRegisterForm {
    pub name:           Option<String> ,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub price:          Option<i32> ,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub category_id:    Option<i32>
}
/// FormをProductに変換する
impl FormToDomain<Product> for ProductRegisterForm {
    fn convert(&self) -> Result<Product, AppError> {
        //  Categoryを生成する
        let category = Category::new(
            CategoryId::try_from(self.category_id.unwrap().clone())?,
            CategoryName::try_from(String::from("dummy"))?);
        //　Productを生成して返す
        Ok(Product::new(
            ProductId::try_from(0)?,
            ProductName::try_from(self.name.as_ref().unwrap().clone())?,
            ProductPrice::try_from(self.price.unwrap().clone())?,
            Some(category)))
    }
}
/// 入力値検証
impl AppValidator for ProductRegisterForm{
    fn validate_value(&self) -> Result<(), ValidationError> {
        let mut errors:HashMap<String,String> = HashMap::new();
        // nameフィールドの検証 未入力と文字数チェック
        if ! validate_length(self.name.as_ref().unwrap(), Some(4), Some(20), None) {
            errors.insert(String::from("name"),String::from("商品名は４文字以上20文字以内で入力して下さい。"));
        }
        // priceフィールドの検証　未入力と範囲チェック
        if ! validate_required(&self.price) {
            errors.insert(String::from("price"),String::from("単価は入力必須です。"));
        }else{
            if ! validate_range(self.price.unwrap(), Some(50), Some(100000) ){
                errors.insert(String::from("price") , String::from("単価は50～100000までで入力して下さい。"));
            }
        }
        // category_idフィールドの検証　未入力と範囲チェック
        if ! validate_required(&self.category_id) {
            errors.insert(String::from("category_id"),String::from("カテゴリは入力必須です。"));
        }else{
            if ! validate_range(self.category_id.unwrap(), Some(1), Some(3) ){
                errors.insert(String::from("category_id"),String::from("不正なカテゴリが選択されました。"));
            }
        }
        if errors.is_empty(){
            Ok(())
        }else{
            Err(ValidationError::from(errors))
        }
    }
}

// 認証
#[derive(Debug , Clone , Deserialize , Serialize , Validate)]
pub struct LoginForm {
    #[validate(length(min = 6 , max = 20, message="ユーザー名は6文字以上20文字以内で入力して下さい。"))]
    pub name:       Option<String> , //  ユーザー名
    #[validate(length(min = 6 , max = 20, message="パスワードは6文字以上20文字以内で入力して下さい。"))]
    pub password:   Option<String>   //  パスワード
}
/// FormをUserに変換する
impl FormToDomain<User> for LoginForm{
    fn convert(&self) -> Result<User, AppError> {
        User::new(UserName::try_from(self.name.as_ref().unwrap().clone())?,
                  Password::try_from(self.password.as_ref().unwrap().clone())?,
                  Mail::try_from(String::from("dummy"))?)
    }
}
/// 入力値検証
impl AppValidator for LoginForm {
    fn validate_value(&self) -> Result<(), ValidationError> {
        // エラーメッセージを格納するHashMap
        let mut errors:HashMap<String,String> = HashMap::new();
        match self.validate() { // 検証メソッドの実行
            Ok(_) => Ok(()) ,
            Err(validate_errors) => {
                // フィールド毎のエラーを取得する
                let field_errors = validate_errors.field_errors();
                // nameフィールドのエラーを取得してHashMapに格納する
                match field_errors.get("name"){
                    Some(name_errors) =>{
                        for name_error in *name_errors {
                            errors.insert(String::from("name"), name_error.message.as_ref().unwrap().to_string());
                        }
                    }, None => ()
                };
                // passwordフィールドのエラーを取得してHashMapに格納する
                match field_errors.get("password"){
                    Some(password_errors)=>{
                        for password_error in *password_errors {
                            errors.insert("password".to_string(), password_error.message.as_ref().unwrap().to_string());
                        }
                    },None => ()
                };
                Err(ValidationError::from(errors)) // 検証エラーを返す
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use anyhow::Result;

    #[test]
    fn search_form_validate() -> Result<()>{
        let form = ProductSearchForm{keyword: Some(String::from(""))};
        let result = form.validate_value();
        println!("{:?}" , result);
        Ok(())
    }

    #[test]
    fn login_form_validate() -> Result<()>{
        let form = LoginForm {
            name: Some(String::from("")) ,
            password: Some(String::from("")) };
        let result = form.validate_value();
        println!("{:?}" , result);
        Ok(())
    }

}