use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::LazyLock,
};

use conversions::TableRow;
use rusqlite::Connection;
use rusqlite_migration::Migrations;
use tables::gates::Gate;
use tables::tiles::Tile;

use crate::types::BoxedError;

pub mod conversions;
pub mod fields;
pub mod tables;

static MIGRATIONS: LazyLock<Migrations<'static>> = LazyLock::new(|| {
    let migrations = vec![Gate::migrations(), Tile::migrations()];
    Migrations::new(migrations.into_iter().flatten().collect())
});

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn establish(file: PathBuf) -> Result<Self, BoxedError> {
        let mut connection = Connection::open(file)?;
        MIGRATIONS.to_latest(&mut connection)?;
        Ok(Self { connection })
    }
}

impl Deref for Database {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

impl DerefMut for Database {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.connection
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
