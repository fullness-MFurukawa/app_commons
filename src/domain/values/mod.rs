pub mod categories;
pub mod products;
pub mod users;

// Value Objectが保持する値を返す
pub trait ValueInto<T> {
    fn value(&self) -> T;
}