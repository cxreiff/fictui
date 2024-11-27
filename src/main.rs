use bevy::prelude::*;

use fictui::play;

fn main() {
    App::new().add_plugins(play::app_plugin).run();
}
