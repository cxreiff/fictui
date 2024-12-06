use std::borrow::Borrow;

use super::{grid_data::GridData, grid_maps::GridMaps};

pub struct Grid {
    pub data: GridData,
    pub maps: GridMaps,
}

impl Grid {
    pub fn init() -> Self {
        let data = GridData::load();

        Self {
            maps: data.borrow().into(),
            data,
        }
    }
}
