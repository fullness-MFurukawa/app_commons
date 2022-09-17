use anyhow::Result;
use crate::domain::values::ValueInto;
use crate::error::AppError;

///
/// ユーザーIDを表す値オブジェクト
///
#[derive(Clone , PartialEq , Eq , Debug)]
pub struct UserId(String);
impl TryFrom<String>  for UserId{
    type Error = AppError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(AppError::from("ユーザーIDがありません。"))
        }else {
            Ok(Self(value))
        }
    }
}
impl ValueInto<String> for UserId{
    fn value(&self) -> String {
        self.0.clone()
    }
}
///
/// ユーザー名を表す値オブジェクト
///
#[derive(Clone , PartialEq , Eq , Debug)]
pub struct UserName(String);
impl TryFrom<String> for UserName{
    type Error = AppError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(AppError::from("ユーザー名がありません。"))
        }else if value.chars().count() > 20 {
            Err(AppError::from("ユーザー名の長さは20文字以内です。"))
        } else {
            Ok(Self(value))
        }
    }
}
impl ValueInto<String> for UserName{
    fn value(&self) -> String {
        self.0.clone()
    }
}
///
/// パスワードを表す値オブジェクト
///
#[derive( Clone , PartialEq , Eq , Debug)]
pub struct Password(String);
impl TryFrom<String> for Password {
    type Error = AppError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(AppError::from("パスワードがありません。"))
        }else {
            Ok(Self(value))
        }
    }
}
impl ValueInto<String> for Password{
    fn value(&self) -> String {
        self.0.clone()
    }
}

///
/// パスワードを表す値オブジェクト
///
#[derive(Clone , PartialEq , Eq , Debug)]
pub struct Mail(String);
impl TryFrom<String> for Mail {
    type Error = AppError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(AppError::from("メールアドレスがありません。"))
        }else if value.chars().count() > 36 {
            Err(AppError::from("メールアドレスの長さは36文字以内です。"))
        }else {
            Ok(Self(value))
        }
    }
}
impl ValueInto<String> for Mail {
    fn value(&self) -> String {
        self.0.clone()
    }
}
