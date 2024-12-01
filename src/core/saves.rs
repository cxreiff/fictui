use bevy::a11y::accesskit::Vec2;

#[derive(Clone)]
pub struct SaveData {
    pub name: String,
    pub _position: Vec2,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            name: "Player".into(),
            _position: Vec2::default(),
        }
    }
}
