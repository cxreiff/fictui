use bevy::prelude::*;
use bevy_ratatui::event::KeyEvent;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use tui_input::backend::crossterm::EventHandler;

use crate::core::commands::command::Command;

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
                    let submitted = interface_state.prompt_input.value();

                    if submitted.is_empty() {
                        return;
                    }

                    prompt_submitted.send(PromptSubmitted(submitted.into()));
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
    for PromptSubmitted(submission) in prompt_submitted.read() {
        interface_state.commands.push(submission.into());

        let response = Command::parse(submission).handle(&interface_state.save_data);

        interface_state.messages.push(response.message);
        interface_state.save_data = response.new_save_data;
    }
}
