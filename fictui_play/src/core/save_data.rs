#[derive(Clone)]
pub struct SaveData {
    pub name: String,
    pub current_tile: u32,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            name: "Player".into(),
            current_tile: 1,
        }
    }
}
