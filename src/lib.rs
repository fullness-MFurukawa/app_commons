pub mod domain;
pub mod service;
pub mod infrastructure;
pub mod application;
pub mod app_commons;
pub mod view_commons;
pub mod error;

///
/// Resultエリアス
///
use crate::error::AppError;
pub type Result<T> = anyhow::Result<T , AppError>;