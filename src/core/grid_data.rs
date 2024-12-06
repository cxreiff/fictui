pub struct GridData {
    pub _id: u32,
    pub _name: String,
    pub tiles: Vec<Tile>,
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
pub struct Item {
    pub id: u32,
    pub _name: String,
    pub _summary: String,
    pub _description: String,
}

impl GridData {
    pub fn load() -> Self {
        // TODO: retrieve from disk

        let tiles = vec![Tile {
            id: 1,
            _name: "first tile".into(),
            summary: "the first tile".into(),
            _description: "you are in a nondescript room".into(),
        }];

        let items = vec![];

        Self {
            _id: 1,
            _name: "the world".into(),
            tiles,
            items,
        }
    }
}
