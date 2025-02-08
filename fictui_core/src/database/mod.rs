use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::LazyLock,
};

use gates::Gate;
use rusqlite::{Connection, Params, Row};
use rusqlite_migration::{Migrations, M};
use tile_instances::TileInstance;
use tiles::Tile;

use crate::types::BoxedError;

pub mod direction;
pub mod gates;
pub mod tile_instances;
pub mod tiles;

static MIGRATIONS: LazyLock<Migrations<'static>> = LazyLock::new(|| {
    let migrations = vec![
        Gate::migrations(),
        Tile::migrations(),
        TileInstance::migrations(),
    ];
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

pub trait TableRow: Sized {
    fn migrations() -> Vec<M<'static>>;
    fn columns() -> &'static [&'static str];
    fn to_params(&self) -> impl Params;
    fn try_from_row(row: &Row) -> rusqlite::Result<Self>;

    fn columns_string() -> String {
        Self::columns().join(", ")
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
