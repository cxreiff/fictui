use std::collections::HashMap;

use super::grid_data::GridData;

pub struct GridMaps {
    pub tiles: HashMap<u32, usize>,
    pub _items: HashMap<u32, usize>,
}

impl From<&GridData> for GridMaps {
    fn from(value: &GridData) -> Self {
        Self {
            tiles: value
                .tiles
                .iter()
                .enumerate()
                .map(|(index, tile)| (tile.id, index))
                .collect(),
            _items: value
                .items
                .iter()
                .enumerate()
                .map(|(index, item)| (item.id, index))
                .collect(),
        }
    }
}
