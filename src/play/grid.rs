use bevy::prelude::*;

use crate::core::grid::Grid;

#[derive(Resource, Deref)]
pub struct GridResource(Grid);

pub fn grid_plugin(app: &mut App) {
    app.init_resource::<GridResource>();
}

impl Default for GridResource {
    fn default() -> Self {
        Self(Grid::init())
    }
}
