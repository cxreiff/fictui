use super::types::Direction;

pub struct GridData {
    pub _id: u32,
    pub _name: String,
    pub tiles: Vec<Tile>,
    pub tile_instances: Vec<TileInstance>,
    pub gates: Vec<Gate>,
    pub items: Vec<Item>,
}

#[derive(Clone)]
pub struct Tile {
    pub id: u32,
    pub _name: String,
    pub summary: String,
    pub _description: String,
}

#[derive(Clone)]
pub struct TileInstance {
    pub id: u32,
    pub resource_id: u32,
}

pub struct Gate {
    pub _id: u32,
    pub tile_instance_id: u32,
    pub next_instance_id: u32,
    pub direction: Direction,
}

#[derive(Clone)]
pub struct Item {
    pub id: u32,
    pub _name: String,
    pub _summary: String,
    pub _description: String,
}

impl GridData {
    pub fn load() -> Self {
        // TODO: retrieve from disk and deserialize with `serde`

        let tiles = vec![
            Tile {
                id: 1,
                _name: "first tile".into(),
                summary: "the first tile".into(),
                _description: "you are in a nondescript room".into(),
            },
            Tile {
                id: 2,
                _name: "second tile".into(),
                summary: "the second tile".into(),
                _description: "you are in a descript room".into(),
            },
        ];

        let items = vec![];

        let tile_instances = vec![
            TileInstance {
                id: 1,
                resource_id: 1,
            },
            TileInstance {
                id: 2,
                resource_id: 2,
            },
        ];

        let gates = vec![
            Gate {
                _id: 1,
                tile_instance_id: 1,
                next_instance_id: 2,
                direction: Direction::North,
            },
            Gate {
                _id: 2,
                tile_instance_id: 2,
                next_instance_id: 1,
                direction: Direction::South,
            },
        ];

        Self {
            _id: 1,
            _name: "the world".into(),
            tiles,
            tile_instances,
            gates,
            items,
        }
    }
}
