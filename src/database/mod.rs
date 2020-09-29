use crate::DbPool;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;

pub mod schema;
pub mod users;

pub fn connect(url: &str) -> Result<DbPool, r2d2::Error> {
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    diesel::r2d2::Pool::builder().build(manager)
}
