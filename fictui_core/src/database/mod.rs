use std::{error::Error, path::PathBuf};

use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub mod models;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub fn establish(file: PathBuf) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> {
        let file = file.to_str().unwrap();
        let mut connection = SqliteConnection::establish(file)?;

        connection.run_pending_migrations(MIGRATIONS)?;

        Ok(Database { connection })
    }
}
