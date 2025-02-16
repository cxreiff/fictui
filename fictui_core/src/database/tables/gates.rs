use rusqlite::{Params, Row};
use rusqlite_migration::M;

use crate::database::{conversions::TableRow, fields::direction::Direction};

#[derive(Debug)]
pub struct Gate {
    pub name: String,
    pub summary: String,
    pub body: String,
    pub source_id: i32,
    pub destination_id: i32,
    pub direction: Direction,
}

impl TableRow for Gate {
    fn name() -> String {
        "gates".into()
    }

    fn columns() -> &'static [&'static str] {
        &[
            "name",
            "summary",
            "body",
            "source_id",
            "destination_id",
            "direction",
        ]
    }

    fn migrations() -> Vec<M<'static>> {
        vec![M::up("
CREATE TABLE gates (
    id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name            VARCHAR NOT NULL,
    summary         VARCHAR NOT NULL,
    body            TEXT NOT NULL,
    source_id       INTEGER NOT NULL,
    destination_id  INTEGER NOT NULL,
    direction       TEXT CHECK(direction IN ('north', 'east', 'south', 'west', 'up', 'down')) NOT NULL,

    FOREIGN KEY(source_id)      REFERENCES tiles(id),
    FOREIGN KEY(destination_id) REFERENCES tiles(id)
)
        ")
        .down("DROP TABLE tiles")]
    }

    fn to_params(&self) -> impl Params {
        (
            &self.name,
            &self.summary,
            &self.body,
            &self.source_id,
            &self.destination_id,
            &self.direction,
        )
    }

    fn try_from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            name: row.get(0)?,
            summary: row.get(1)?,
            body: row.get(2)?,
            source_id: row.get(3)?,
            destination_id: row.get(4)?,
            direction: row.get(5)?,
        })
    }
}
