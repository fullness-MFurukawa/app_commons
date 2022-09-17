use std::env;
use std::sync::Arc;
use std::time::Duration;
use dotenv::dotenv;
use async_trait::async_trait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use crate::infrastructure::pool::PoolProvider;

pub struct SeaOrmPool;
#[async_trait]
impl PoolProvider<Arc<DatabaseConnection>> for SeaOrmPool{
    async fn get() -> Arc<DatabaseConnection> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").unwrap();
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);
        let connection = Database::connect(opt).await.unwrap();
        Arc::new(connection)
    }
}