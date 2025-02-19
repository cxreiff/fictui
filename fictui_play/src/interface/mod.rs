use std::io;

use bevy::{
    app::{App, PostStartup, Update},
    prelude::{IntoSystem, ResMut, Resource},
    utils::error,
};
use bevy_ratatui::terminal::RatatuiContext;
use fictui_core::{aux_data::AuxData, save_data::SaveData};
use ratatui::layout::Size;
use tui_input::Input;
use tui_scrollview::ScrollViewState;

pub mod layout;
pub mod map;
pub mod prompt;
pub mod readout;
pub mod utils;

#[derive(Resource, Default)]
pub struct InterfaceState {
    pub _focus: InterfaceFocus,
    pub prompt_input: Input,
    pub readout_scroll: ScrollViewState,
    previous_content_size: Size,
    previous_scroll_view_size: Size,
    pub commands: Vec<String>,
    pub messages: Vec<String>,
    pub save_data: SaveData,
    pub aux_data: Option<AuxData>,
}

#[derive(Default)]
pub enum InterfaceFocus {
    #[default]
    Prompt,
}

pub fn interface_plugin(app: &mut App) {
    app.add_systems(PostStartup, clear_terminal_system.map(error))
        .add_systems(Update, draw_scene_system.map(error))
        .init_resource::<InterfaceState>();
}

fn clear_terminal_system(mut ratatui: ResMut<RatatuiContext>) -> io::Result<()> {
    // Some terminal emulators (e.g. Ghostty) seem not to clear the terminal when entering raw
    // mode. Rather than fixing this in individual terminal emulators, we can clear on startup to
    // guarantee a blank starting point.
    ratatui.clear()?;
    Ok(())
}

fn draw_scene_system(
    mut ratatui: ResMut<RatatuiContext>,
    mut interface_state: ResMut<InterfaceState>,
) -> io::Result<()> {
    ratatui.draw(|frame| interface_state.draw_layout(frame, frame.area()))?;

    Ok(())
}
