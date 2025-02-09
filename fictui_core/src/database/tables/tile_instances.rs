use rusqlite::{Params, Row};
use rusqlite_migration::M;

use crate::database::TableRow;

pub struct TileInstance {
    pub id: i32,
    pub tile_id: i32,
}

impl TableRow for TileInstance {
    fn columns() -> &'static [&'static str] {
        &["id", "tile_id"]
    }

    fn migrations() -> Vec<M<'static>> {
        vec![M::up(
            "
CREATE TABLE tile_instances (
    id      INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    tile_id INTEGER NOT NULL,

    FOREIGN KEY(tile_id) REFERENCES tiles(id)
)
            ",
        )
        .down("DROP TABLE tile_instances")]
    }

    fn to_params(&self) -> impl Params {
        (&self.id, &self.tile_id)
    }

    fn try_from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            tile_id: row.get(1)?,
        })
    }
}
