use bevy::app::App;

mod handler;
mod interface;
mod plugin;
mod prompt;

pub fn run() {
    App::new().add_plugins(plugin::app_plugin).run();
}
