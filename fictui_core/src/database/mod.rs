use std::{error::Error, path::PathBuf, sync::LazyLock};

use rusqlite::{Connection, Params, Result, Row};
use rusqlite_migration::{Migrations, M};

mod tile;

pub use tile::Tile;
use tile::TileInstance;

static MIGRATIONS: LazyLock<Migrations<'static>> = LazyLock::new(|| {
    let migrations = vec![Tile::migrations(), TileInstance::migrations()];
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

pub trait TableRow {
    fn name() -> String;

    fn columns() -> &'static [&'static str];

    fn to_params(&self) -> impl Params;

    fn try_from_row(row: &Row) -> Result<Self>
    where
        Self: Sized;

    fn migrations() -> Vec<M<'static>>;

    fn generate_insert_sql() -> String {
        let table = Self::name();
        let columns = Self::columns().join(", ");
        let placeholders = (1..=Self::columns().len())
            .map(|i| format!("?{i}"))
            .collect::<Vec<_>>()
            .join(", ");

        format!("INSERT INTO {table} ({columns}) VALUES ({placeholders})")
    }

    fn generate_list_sql() -> String {
        let table = Self::name();
        let columns = Self::columns().join(", ");
        format!("SELECT {columns} FROM {table}")
    }

    fn generate_select_sql() -> String {
        let list_query = Self::generate_list_sql();
        format!("{list_query} WHERE id = ?1")
    }

    fn insert(db: &Database, row: Self) -> Result<i64>
    where
        Self: Sized,
    {
        db.connection
            .execute(&Self::generate_insert_sql(), row.to_params())?;

        Ok(db.connection.last_insert_rowid())
    }

    fn select(db: &Database, id: i64) -> Result<Self>
    where
        Self: Sized,
    {
        db.connection
            .prepare(&Self::generate_select_sql())?
            .query_row([id], Self::try_from_row)
    }

    fn list(db: &Database) -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        db.connection
            .prepare(&Self::generate_list_sql())?
            .query_map([], Self::try_from_row)?
            .collect()
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
