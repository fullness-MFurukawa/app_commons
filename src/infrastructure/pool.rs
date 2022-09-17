use async_trait::async_trait;
#[async_trait]
pub trait PoolProvider<T>{
    async fn get() -> T;
}