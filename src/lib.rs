pub mod error;

///
/// Resultエリアス
///
use crate::error::AppError;
pub type Result<T> = anyhow::Result<T , AppError>;