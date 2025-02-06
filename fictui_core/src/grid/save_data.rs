#[derive(Clone)]
pub struct SaveData {
    pub name: String,
    pub current_tile: i32,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            name: "Player".into(),
            current_tile: 1,
        }
    }
}
