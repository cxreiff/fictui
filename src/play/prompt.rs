use bevy::prelude::*;
use bevy_ratatui::event::KeyEvent;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use tui_input::backend::crossterm::EventHandler;

use crate::core::command::handle_command;

use super::interface::InterfaceState;

#[derive(Event, Deref, DerefMut)]
pub struct PromptSubmitted(pub String);

pub fn prompt_plugin(app: &mut App) {
    app.add_event::<PromptSubmitted>()
        .add_systems(PreUpdate, handle_keyboard_system)
        .add_systems(Update, handle_prompt_submissions_system);
}

fn handle_keyboard_system(
    mut ratatui_events: EventReader<KeyEvent>,
    mut prompt_submitted: EventWriter<PromptSubmitted>,
    mut interface_state: ResMut<InterfaceState>,
) {
    for key_event in ratatui_events.read() {
        match key_event.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => match key_event.code {
                KeyCode::Enter => {
                    prompt_submitted
                        .send(PromptSubmitted(interface_state.prompt_input.value().into()));
                    interface_state.prompt_input.reset();
                }
                KeyCode::Esc => {
                    interface_state.prompt_input.reset();
                }
                _ => {
                    interface_state
                        .prompt_input
                        .handle_event(&Event::Key(key_event.0));
                }
            },
            _ => {}
        }
    }
}

fn handle_prompt_submissions_system(
    mut prompt_submitted: EventReader<PromptSubmitted>,
    mut interface_state: ResMut<InterfaceState>,
) {
    for submission in prompt_submitted.read() {
        let response = handle_command(submission, &interface_state.save_data);
        interface_state.commands.push(response.command);
        interface_state.messages.push(response.message);
        interface_state.save_data = response.new_save_data;
    }
}
