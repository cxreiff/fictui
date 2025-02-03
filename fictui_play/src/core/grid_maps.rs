use std::collections::HashMap;

use super::grid_data::GridData;

pub struct GridMaps {
    pub tiles: HashMap<u32, usize>,
    pub tile_instances: HashMap<u32, usize>,
    pub gates_by_tile_instance: HashMap<u32, Vec<usize>>,
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
            tile_instances: value
                .tile_instances
                .iter()
                .enumerate()
                .map(|(index, tile_instance)| (tile_instance.id, index))
                .collect(),
            gates_by_tile_instance: value.gates.iter().enumerate().fold(
                HashMap::new(),
                |mut acc, (index, gate)| {
                    acc.entry(gate.tile_instance_id)
                        .and_modify(|e| e.push(index))
                        .or_insert(vec![index]);
                    acc
                },
            ),
            _items: value
                .items
                .iter()
                .enumerate()
                .map(|(index, item)| (item.id, index))
                .collect(),
        }
    }
}
