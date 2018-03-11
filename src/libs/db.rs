use r2d2_diesel::ConnectionManager;

use diesel::sqlite::SqliteConnection;

use super::conn::Conn;

pub type DbConnection = SqliteConnection;

pub type ConnPool = Conn<ConnectionManager<DbConnection>>;
