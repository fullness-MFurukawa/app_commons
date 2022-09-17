use anyhow::Result;
use crate::domain::values::ValueInto;
use crate::error::AppError;

///
/// 商品番号を表す値オブジェクト
///
#[derive(Clone , PartialEq , Eq , Debug)]
pub struct ProductId(i32);
impl TryFrom<i32> for ProductId {
    type Error = AppError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(AppError::from("不正な商品番号です。"))
        } else {
            Ok(Self(value))
        }
    }
}
impl ValueInto<i32> for ProductId{
    fn value(&self) -> i32 {
        self.0.clone()
    }
}

///
/// 商品名を表す値オブジェクト
///
#[derive(Clone , PartialEq , Eq , Debug)]
pub struct ProductName(String);
impl TryFrom<String> for ProductName{
    type Error = AppError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(AppError::from("商品名がありません。"))
        }else if value.chars().count() > 20 {
            Err(AppError::from("商品名の長さは20文字以内です。"))
        }else {
            Ok(Self(value))
        }
    }
}
impl ValueInto<String> for ProductName{
    fn value(&self) -> String {
        self.0.clone()
    }
}

///
/// 単価を表す値オブジェクト
///
#[derive(Clone , Copy , PartialEq , Eq , Debug)]
pub struct ProductPrice(i32);
impl TryFrom<i32> for ProductPrice{
    type Error = AppError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 50 && value <= 10000 {
            Ok(Self(value))
        }else{
            Err(AppError::from("不正な単価です。"))
        }
    }
}
impl ValueInto<i32> for ProductPrice{
    fn value(&self) -> i32 {
        self.0.clone()
    }
}
