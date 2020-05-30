//! Db executor actor
use crate::utils::env::ENV;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

fn init_pool(database_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn connect() -> DbPool {
    init_pool(&ENV.database_url).expect("Failed to create pool")
}
