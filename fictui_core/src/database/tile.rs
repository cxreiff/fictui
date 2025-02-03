use rusqlite::Result;
use rusqlite_migration::M;

use super::Database;

#[derive(Debug)]
pub struct Tile {
    pub id: i32,
    pub name: String,
}

pub fn migrations() -> Vec<M<'static>> {
    vec![
        M::up(
            "CREATE TABLE tiles (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL
            )",
        )
        .down("DROP TABLE tiles"),
        M::up(
            "CREATE TABLE tile_instances (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL
            )",
        )
        .down("DROP TABLE tile_instances"),
    ]
}

impl Database {
    pub fn create_tile(&self, row: Tile) -> Result<()> {
        self.connection.execute(
            "INSERT INTO tiles (id, name) VALUES (?1, ?2)",
            (&row.id, &row.name),
        )?;
        Ok(())
    }

    pub fn list_tiles(&self) -> Result<Vec<Tile>> {
        self.connection
            .prepare("SELECT id, name FROM tiles")?
            .query_map([], |row| {
                Ok(Tile {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            })?
            .collect()
    }
}
