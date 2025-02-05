use diesel::{
    prelude::{Insertable, Queryable},
    query_dsl::methods::SelectDsl,
    sqlite::Sqlite,
    RunQueryDsl, Selectable, SelectableHelper,
};

use super::super::Database;

diesel::table! {
    tiles (id) {
        id -> Integer,
        name -> Text,
        summary -> Text,
        body -> Text,
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = tiles)]
#[diesel(check_for_backend(Sqlite))]
pub struct Tile {
    pub id: i32,
    pub name: String,
    pub summary: String,
    pub body: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = tiles)]
pub struct NewTile<'a> {
    pub name: &'a str,
    pub summary: &'a str,
    pub body: &'a str,
}

impl Database {
    pub fn insert_tile(&mut self, new_tile: NewTile) -> Tile {
        diesel::insert_into(tiles::table)
            .values(&new_tile)
            .returning(Tile::as_returning())
            .get_result(&mut self.connection)
            .expect("error creating new tile")
    }

    pub fn list_tiles(&mut self) -> Vec<Tile> {
        tiles::table
            .select(Tile::as_select())
            .load(&mut self.connection)
            .expect("error listing tiles")
    }
}
