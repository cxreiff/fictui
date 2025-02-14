use rusqlite::Row;
use rusqlite_migration::M;

use crate::database::TableRow;

#[derive(Debug)]
pub struct Tile {
    pub id: i32,
    pub name: String,
    pub summary: String,
    pub body: String,
}

impl TableRow for Tile {
    fn columns() -> &'static [&'static str] {
        &["id", "name", "summary", "body"]
    }

    fn migrations() -> Vec<M<'static>> {
        vec![M::up(
            "
CREATE TABLE tiles (
    id        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name      VARCHAR NOT NULL,
    summary   VARCHAR NOT NULL,
    body TEXT NOT NULL
)
            ",
        )
        .down("DROP TABLE tiles")]
    }

    fn to_params(&self) -> impl rusqlite::Params {
        (&self.id, &self.name, &self.summary, &self.body)
    }

    fn try_from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: row.get(1)?,
            summary: row.get(2)?,
            body: row.get(3)?,
        })
    }
}
