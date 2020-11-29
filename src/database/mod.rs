use crate::DbPool;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;

/// Diesel generated schemas
pub mod schema;

/// Database functions for handling users
pub mod users;

/// Connect to database at `url`.
pub fn connect(url: &str) -> Result<DbPool, r2d2::Error> {
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    diesel::r2d2::Pool::builder().build(manager)
}
