use std::io;

use bevy::prelude::*;
use bevy::utils::error;
use bevy_ratatui::terminal::RatatuiContext;
use ratatui::layout::{Constraint, Layout, Position};

use super::{prompt::Prompt, readout::Readout};

pub fn interface_plugin(app: &mut App) {
    app.add_systems(Update, draw_scene_system.map(error))
        .init_resource::<InterfaceState>();
}

#[derive(Resource, Default)]
pub struct InterfaceState {
    //
}

#[derive(Default)]
pub struct FrameState {
    pub cursor_hint: (u16, u16),
}

fn draw_scene_system(
    mut ratatui: ResMut<RatatuiContext>,
    readout: Res<Readout>,
    prompt: Res<Prompt>,
) -> io::Result<()> {
    ratatui.draw(|frame| {
        let mut frame_state = FrameState::default();

        let constraints = [Constraint::Min(1), Constraint::Length(3)];
        let layout = Layout::vertical(constraints).split(frame.area());

        frame.render_widget(readout.as_ref(), layout[0]);
        frame.render_stateful_widget(prompt.as_ref(), layout[1], &mut frame_state);

        let (cursor_x, cursor_y) = frame_state.cursor_hint;
        frame.set_cursor_position(Position::new(cursor_x, cursor_y));
    })?;

    Ok(())
}
