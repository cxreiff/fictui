#[derive(Clone)]
pub struct SaveData {
    pub name: String,
    pub coords: (u32, u32, u32),
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            name: "Player".into(),
            coords: (0, 0, 0),
        }
    }
}
