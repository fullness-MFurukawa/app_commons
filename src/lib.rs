pub mod domain;
pub mod service;
pub mod infrastructure;
pub mod application;
pub mod presentation;
pub mod error;

///
/// Resultエリアス
///
use crate::error::AppError;
pub type Result<T> = anyhow::Result<T , AppError>;