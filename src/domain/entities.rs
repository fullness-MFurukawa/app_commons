use easy_hasher::easy_hasher::sha3_512;
use uuid::Uuid;
use crate::domain::values::products::{ProductId, ProductName, ProductPrice};
use crate::domain::values::categories::{CategoryId, CategoryName};
use crate::domain::values::users::{Mail, Password, UserId, UserName};
use crate::domain::values::ValueInto;
use crate::Result;


///
///  trait:識別子操の特性
///
pub trait Characteristic {
    type Identifier;      // 識別子
    /// 識別子を変更する
    fn change(&mut self , value: &Self::Identifier) -> Result<()>;
    /// 識別子を取得する
    fn get(&self) -> Self::Identifier;
    /// 識別子の同一性を検証する
    fn equals(&self , value: &Self::Identifier) -> bool;
}

///
/// カテゴリを表すDomain Entity
///
#[derive(Clone , PartialEq , Eq , Debug)]
pub struct Category{
    id:         CategoryId ,
    pub name:   CategoryName
}
impl Category {
    pub fn new(id: CategoryId , name: CategoryName) -> Self{
        Self {id, name}
    }
}
impl  Characteristic for Category{
    type Identifier = CategoryId;
    fn change(&mut self, value: &Self::Identifier) -> Result<()> {
        self.id = value.clone();
        Ok(())
    }
    fn get(&self) -> Self::Identifier {
        self.id.clone()
    }
    fn equals(&self, value: &Self::Identifier) -> bool {
        self.id.eq(value)
    }
}

///
/// 商品を表すDomain Entity
///
#[derive(Clone , PartialEq , Eq , Debug)]
pub struct Product{
    id:                 ProductId ,         // 商品番号
    pub name:           ProductName ,       // 商品名
    pub price:          ProductPrice ,      // 商品単価
    pub category:       Option<Category>    // カテゴリ
}
impl Product {
    pub fn new(id: ProductId, name: ProductName, price: ProductPrice , category: Option<Category>) -> Self {
        Self{ id , name , price , category }
    }
}
impl Characteristic for Product {
    type Identifier = ProductId;
    fn change(&mut self, value: &Self::Identifier) -> Result<()> {
        self.id = value.clone();
        Ok(())
    }
    fn get(&self) -> Self::Identifier {
        self.id.clone()
    }
    fn equals(&self, value: &Self::Identifier) -> bool {
        self.id.eq(&value)
    }
}

///
/// ユーザーを表す Domain Entity
///
#[derive(Clone , PartialEq , Eq , Debug)]
pub struct User {
    user_id:        UserId ,
    pub user_name:  UserName ,
    pub password:   Password ,
    pub mail:       Mail
}
impl User {
    /// 値を生成する
    /// user_idの生成、パスワードのハッシュ変換をする
    pub fn new(user_name: UserName , password:Password , mail:Mail) -> Result<Self> {
        // uuidでユーザーIdを生成する
        let _user_id = Uuid::new_v4().to_string();
        // 受け取ったパスワードをSHA3-512でハッシュ変換する
        let _password = sha3_512(&password.value()).to_hex_string();
        // 値を生成した結果を返す
        Ok(Self {user_id: UserId::try_from(_user_id)? , user_name,
            password: Password::try_from(_password)? , mail})
    }
    /// すべての値を受け取って値を生成する
    pub fn rebuilding(user_id: UserId , user_name: UserName , password: Password , mail: Mail) -> Self{
        Self{user_id,user_name,password,mail}
    }
}
impl Characteristic for User {
    type Identifier = UserId;
    fn change(&mut self, value: &Self::Identifier) -> Result<()> {
        self.user_id = value.clone();
        Ok(())
    }
    fn get(&self) -> Self::Identifier {
        self.user_id.clone()
    }
    fn equals(&self, value: &Self::Identifier) -> bool {
        self.user_id.eq(value)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use anyhow::Result;
    #[test]
    fn category()  -> Result<()> {
        let category1 = Category::new(CategoryId::try_from(1)?,
                                      CategoryName::try_from(String::from("文房具"))?);
        let category2 = Category::new(CategoryId::try_from(1)?,
                                      CategoryName::try_from(String::from("文房具"))?);
        let result = category1.equals(&category2.get());
        println!("result = {:?}" , result);
        let category3 = Category::new(CategoryId::try_from(1)?,
                                      CategoryName::try_from(String::from("文房具"))?);
        let category4 = Category::new(CategoryId::try_from(2)?,
                                      CategoryName::try_from(String::from("雑貨"))?);
        let result = category3.equals(&category4.get());
        println!("result = {:?}" , result);
        Ok(())
    }
}