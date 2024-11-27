use std::io;

use bevy::prelude::*;
use bevy::utils::error;
use bevy_ratatui::terminal::RatatuiContext;
use ratatui::layout::{Constraint, Layout};

use super::{prompt::Prompt, readout::Readout};

pub fn interface_plugin(app: &mut App) {
    app.add_systems(Update, draw_scene_system.map(error))
        .init_resource::<InterfaceState>();
}

#[derive(Resource, Default)]
struct InterfaceState {
    //
}

fn draw_scene_system(
    mut ratatui: ResMut<RatatuiContext>,
    readout: Res<Readout>,
    prompt: Res<Prompt>,
) -> io::Result<()> {
    ratatui.draw(|frame| {
        let constraints = [Constraint::Min(1), Constraint::Length(5)];
        let layout = Layout::vertical(constraints).split(frame.area());

        frame.render_widget(readout.as_ref(), layout[0]);
        frame.render_widget(prompt.as_ref(), layout[1]);
    })?;

    Ok(())
}
