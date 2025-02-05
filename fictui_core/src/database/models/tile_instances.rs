use diesel::{
    prelude::{Insertable, Queryable},
    query_dsl::methods::SelectDsl,
    sqlite::Sqlite,
    RunQueryDsl, Selectable, SelectableHelper,
};

use super::super::Database;

diesel::table! {
    tile_instances (id) {
        id -> Integer,
        tile_id -> Integer,
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = tile_instances)]
#[diesel(check_for_backend(Sqlite))]
pub struct TileInstance {
    pub id: i32,
    pub tile_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = tile_instances)]
pub struct NewTileInstance<'a> {
    pub tile_id: &'a i32,
}

impl Database {
    pub fn insert_tile_instance(&mut self, new_tile_instance: NewTileInstance) -> TileInstance {
        diesel::insert_into(tile_instances::table)
            .values(&new_tile_instance)
            .returning(TileInstance::as_returning())
            .get_result(&mut self.connection)
            .expect("error creating new tile instance")
    }

    pub fn list_tile_instances(&mut self) -> Vec<TileInstance> {
        tile_instances::table
            .select(TileInstance::as_select())
            .load(&mut self.connection)
            .expect("error listing tile instances")
    }
}
