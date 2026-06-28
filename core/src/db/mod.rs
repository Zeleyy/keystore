use std::path::{PathBuf};
use include_dir::{Dir, include_dir};
use rusqlite_migration::Migrations;

pub mod pool;
mod models;
mod repository;

pub use pool::{DbPool, DbConn, get_connection, create_pool};

pub mod category {
    pub use super::models::category::*;
    pub use super::repository::category::*;
}

pub mod entry {
    pub use super::models::entry::*;
    pub use super::repository::entry::*;
}

pub mod history {
    pub use super::models::history::*;
    pub use super::repository::history::*;
}

pub mod tag {
    pub use super::models::tag::*;
    pub use super::repository::tag::*;
}

static MIGRATIONS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/migrations");

pub fn init_db(db_path: PathBuf, encryption_key: String) -> DbPool {
    let pool = create_pool(db_path, encryption_key);
    
    let mut conn = get_connection(&pool).unwrap();
    
    let migrations = Migrations::from_directory(&MIGRATIONS_DIR)
        .expect("Failed to parse migrations from directory");
    
    migrations
        .to_latest(&mut conn)
        .expect("Failed to run database migrations");

    pool
}
