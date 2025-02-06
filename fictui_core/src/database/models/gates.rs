use diesel::{
    prelude::{Insertable, Queryable},
    query_dsl::methods::SelectDsl,
    sqlite::Sqlite,
    RunQueryDsl, Selectable, SelectableHelper,
};
use diesel_derive_enum::DbEnum;

use super::super::Database;

#[derive(DbEnum, Debug)]
pub enum GateDirection {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

diesel::table! {
    use diesel::sql_types::Integer;
    use diesel::sql_types::Text;
    use super::GateDirectionMapping;

    gates (id) {
        id -> Integer,
        name -> Text,
        summary -> Text,
        body -> Text,
        source_tile_id -> Integer,
        destination_tile_id -> Integer,
        direction -> GateDirectionMapping,
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = gates)]
#[diesel(check_for_backend(Sqlite))]
pub struct Gate {
    pub id: i32,
    pub name: String,
    pub summary: String,
    pub body: String,
    pub source_tile_id: i32,
    pub destination_tile_id: i32,
    pub direction: GateDirection,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = gates)]
pub struct NewGate<'a> {
    pub name: &'a str,
    pub summary: &'a str,
    pub body: &'a str,
    pub source_tile_id: &'a i32,
    pub destination_tile_id: &'a i32,
    pub direction: &'a GateDirection,
}

impl Database {
    pub fn insert_gate(&mut self, new_gate: NewGate) -> Gate {
        diesel::insert_into(gates::table)
            .values(&new_gate)
            .returning(Gate::as_returning())
            .get_result(&mut self.connection)
            .expect("error creating new gate")
    }

    pub fn list_gates(&mut self) -> Vec<Gate> {
        gates::table
            .select(Gate::as_select())
            .load(&mut self.connection)
            .expect("error listing gates")
    }
}
