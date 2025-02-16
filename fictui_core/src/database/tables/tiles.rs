use rusqlite::Row;
use rusqlite_migration::M;

use crate::database::conversions::TableRow;

#[derive(Debug)]
pub struct Tile {
    pub name: String,
    pub summary: String,
    pub body: String,
}

impl TableRow for Tile {
    fn name() -> String {
        "tiles".into()
    }

    fn columns() -> &'static [&'static str] {
        &["name", "summary", "body"]
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
        (&self.name, &self.summary, &self.body)
    }

    fn try_from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            name: row.get(0)?,
            summary: row.get(1)?,
            body: row.get(2)?,
        })
    }
}
