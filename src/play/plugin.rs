use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_ratatui::RatatuiPlugins;

use super::{interface, prompt, readout};

pub fn app_plugin(app: &mut App) {
    app.add_plugins((
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(
            1. / 60.,
        ))),
        RatatuiPlugins::default(),
    ))
    .add_plugins((
        interface::interface_plugin,
        prompt::prompt_plugin,
        readout::readout_plugin,
    ));
}
