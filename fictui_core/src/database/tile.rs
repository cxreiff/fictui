use rusqlite::{Params, Result, Row};
use rusqlite_migration::M;

use super::TableRow;

#[derive(Debug)]
pub struct Tile {
    pub name: String,
    pub body: String,
}

impl TableRow for Tile {
    fn name() -> String {
        "tiles".into()
    }

    fn columns() -> &'static [&'static str] {
        &["name", "body"]
    }

    fn to_params(&self) -> impl Params {
        (&self.name, &self.body)
    }

    fn try_from_row(row: &Row) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            name: row.get(0)?,
            body: row.get(1)?,
        })
    }

    fn migrations() -> Vec<M<'static>> {
        vec![M::up(
            "CREATE TABLE tiles (
                id    INTEGER PRIMARY KEY AUTOINCREMENT,
                name  TEXT NOT NULL,
                body  TEXT
            )",
        )
        .down("DROP TABLE tiles")]
    }
}

pub struct TileInstance {
    resource_id: i64,
}

impl TableRow for TileInstance {
    fn name() -> String {
        "tile_instances".into()
    }

    fn columns() -> &'static [&'static str] {
        &["resource_id"]
    }

    fn to_params(&self) -> impl Params {
        (&self.resource_id,)
    }

    fn try_from_row(row: &Row) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            resource_id: row.get(0)?,
        })
    }

    fn migrations() -> Vec<M<'static>> {
        vec![M::up(
            "CREATE TABLE tile_instances (
                id           INTEGER PRIMARY KEY,
                resource_id  INTEGER NOT NULL REFERENCES tiles(id)
            )",
        )
        .down("DROP TABLE tile_instances")]
    }
}
