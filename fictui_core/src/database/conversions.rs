use rusqlite::{Connection, Params, Row};
use rusqlite_migration::M;

pub trait TableRow: Sized {
    fn name() -> String;
    fn migrations() -> Vec<M<'static>>;
    fn columns() -> &'static [&'static str];
    fn to_params(&self) -> impl Params;
    fn try_from_row(row: &Row) -> rusqlite::Result<Self>;

    fn columns_string() -> String {
        Self::columns().join(", ")
    }

    fn values_slots() -> String {
        (1..=Self::columns().len())
            .map(|i| format!("?{i}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

pub trait Retrievable: Sized {
    fn retrieve(connection: &Connection, id: i32) -> rusqlite::Result<Self>;
}

impl<T: TableRow> Retrievable for T {
    fn retrieve(connection: &Connection, id: i32) -> rusqlite::Result<Self> {
        connection.query_row(
            &format!(
                "SELECT {} FROM {} WHERE id = ?1",
                Self::columns_string(),
                Self::name(),
            ),
            (id,),
            Self::try_from_row,
        )
    }
}

pub trait Storable: Sized {
    fn store(self, connection: &Connection) -> rusqlite::Result<i32>;
}

impl<T: TableRow> Storable for T {
    fn store(self, connection: &Connection) -> rusqlite::Result<i32> {
        connection.execute(
            &format!(
                "INSERT INTO {} ({}) VALUES ({})",
                Self::name(),
                Self::columns_string(),
                Self::values_slots(),
            ),
            self.to_params(),
        )?;

        Ok(connection.last_insert_rowid() as i32)
    }
}

pub trait Countable: Sized {
    fn count(connection: &Connection) -> rusqlite::Result<usize>;
}

impl<T: TableRow> Countable for T {
    fn count(connection: &Connection) -> rusqlite::Result<usize> {
        connection.query_row(
            &format!("SELECT COUNT(*) FROM {}", Self::name()),
            (),
            |row| row.get(0),
        )
    }
}
