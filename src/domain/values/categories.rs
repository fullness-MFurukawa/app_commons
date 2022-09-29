use crate::domain::values::ValueInto;
use crate::{Result,AppError};

///
///  カテゴリ番号を表す値オブジェクト
///
#[derive(Clone , Debug , PartialEq , Eq)]
pub struct CategoryId(i32);
// 値を生成して返す、ルール違反の場合はAppErrorを返す
impl TryFrom<i32> for CategoryId{
    type Error = AppError;
    fn try_from(value: i32) -> Result<Self> {
        if value >= 1 && value <= 3 {
            Ok(Self(value))
        }else {
            Err(AppError::from("不正な商品カテゴリ番号です。"))
        }
    }
}
// 保持している値を返す
impl ValueInto<i32> for CategoryId{
    fn value(&self) -> i32 {
        self.0.clone()
    }
}

///
/// 商品カテゴリ名を表す値オブジェクト
///
#[derive(Clone , Debug , PartialEq , Eq)]
pub struct CategoryName(String);
// 値を生成して返す、ルール違反の場合はAppErrorを返す
impl TryFrom<String> for CategoryName{
    type Error = AppError;
    fn try_from(value: String) -> Result<Self> {
        if value.is_empty() {
            Err(AppError::from("カテゴリ名がありません。"))
        }else if value.chars().count() > 20 {
            Err(AppError::from("カテゴリ名の長さは20文字以内です。"))
        }else {
            Ok(Self(value))
        }
    }
}
// 保持している値を返す
impl ValueInto<String> for CategoryName{
    fn value(&self) -> String {
        self.0.clone()
    }
}