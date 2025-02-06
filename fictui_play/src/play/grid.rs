use bevy::prelude::*;
use fictui_core::grid::Grid;

#[derive(Deref, DerefMut)]
pub struct GridResource(Grid);

pub fn grid_plugin(app: &mut App) {
    app.init_non_send_resource::<GridResource>();
}

impl Default for GridResource {
    fn default() -> Self {
        Self(Grid::build("test.fic".into()).unwrap())
    }
}
