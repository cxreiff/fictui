use bevy::utils::hashbrown::HashMap;

pub struct GridData {
    pub _id: u32,
    pub _name: String,
    pub tiles: Vec<Tile>,
    pub items: Vec<Item>,
    pub tiles_map: HashMap<(u32, u32, u32), u32>,
}

#[derive(Clone)]
pub struct Tile {
    pub id: u32,
    pub _name: String,
    pub summary: String,
    pub _description: String,
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

        let tiles_map = HashMap::from([((0, 0, 0), 1), ((0, 1, 0), 2)]);

        Self {
            _id: 1,
            _name: "the world".into(),
            tiles,
            items,
            tiles_map,
        }
    }
}
