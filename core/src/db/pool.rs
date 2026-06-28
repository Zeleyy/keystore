use std::path::PathBuf;

use r2d2::{ManageConnection, Pool};
use rusqlite::Connection;

pub type DbPool = Pool<SqliteConnectionManager>;
pub type DbConn = r2d2::PooledConnection<SqliteConnectionManager>;

pub struct SqliteConnectionManager {
    db_path: PathBuf,
    encryption_key: String,
}

impl SqliteConnectionManager {
    pub fn new(path: PathBuf, encryption_key: String) -> Self {
        Self {
            db_path: path,
            encryption_key,
        }
    }
}

impl ManageConnection for SqliteConnectionManager {
    type Connection = Connection;
    type Error = rusqlite::Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let conn = Connection::open(&self.db_path)?;

        let pragma_key_cmd = format!("PRAGMA key = x'{}';", self.encryption_key);
        conn.execute_batch(&pragma_key_cmd)?;

        conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
        Ok(conn)
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.pragma_update(None, "user_version", 0)
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}

pub fn create_pool(db_path: PathBuf, encryption_key: String) -> DbPool {
    let manager = SqliteConnectionManager::new(db_path, encryption_key);
    Pool::builder()
        .max_size(10)
        .min_idle(Some(2))
        .build(manager)
        .expect("Faild to create pool")
}

pub fn get_connection(pool: &DbPool) -> Result<DbConn, String> {
    pool.get().map_err(|e| format!("Database connection error: {}", e))
}
