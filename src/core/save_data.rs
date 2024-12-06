#[derive(Clone)]
pub struct SaveData {
    pub name: String,
    pub tile: u32,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            name: "Player".into(),
            tile: 1,
        }
    }
}
