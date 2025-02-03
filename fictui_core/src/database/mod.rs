use std::{error::Error, path::PathBuf, sync::LazyLock};

use rusqlite::{Connection, Result};
use rusqlite_migration::Migrations;

mod tile;

pub use tile::Tile;

static MIGRATIONS: LazyLock<Migrations<'static>> = LazyLock::new(|| {
    let migrations = vec![tile::migrations()];
    Migrations::new(migrations.into_iter().flatten().collect())
});

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn establish(file: PathBuf) -> Result<Self, Box<dyn Error>> {
        let mut connection = Connection::open(file)?;
        MIGRATIONS.to_latest(&mut connection)?;
        Ok(Self { connection })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_migrations() {
        assert!(MIGRATIONS.validate().is_ok());
    }
}
