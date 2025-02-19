use bevy::prelude::*;
use fictui_core::handler::Handler;

#[derive(Deref, DerefMut)]
pub struct HandlerResource(Handler);

pub fn handler_plugin(app: &mut App) {
    app.init_non_send_resource::<HandlerResource>();
}

impl Default for HandlerResource {
    fn default() -> Self {
        Self(Handler::build("test.fic".into()).unwrap())
    }
}
