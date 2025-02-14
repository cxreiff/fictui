use bevy::prelude::*;
use bevy_ratatui::event::KeyEvent;
use fictui_core::parser::Command;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use tui_input::backend::crossterm::EventHandler;

use super::{handler::HandlerResource, interface::InterfaceState};

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
                    interface_state.readout_scroll.scroll_to_bottom();
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
                KeyCode::Up => {
                    interface_state.readout_scroll.scroll_up();
                }
                KeyCode::Down => {
                    interface_state.readout_scroll.scroll_down();
                }
                KeyCode::PageUp => {
                    interface_state.readout_scroll.scroll_page_up();
                }
                KeyCode::PageDown => {
                    interface_state.readout_scroll.scroll_page_down();
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
    mut app_exit: EventWriter<AppExit>,
    mut interface_state: ResMut<InterfaceState>,
    mut grid: NonSendMut<HandlerResource>,
) {
    for PromptSubmitted(submission) in prompt_submitted.read() {
        interface_state.commands.push(submission.into());

        let command = Command::parse(submission);

        if let Command::Quit = command {
            app_exit.send_default();
            return;
        }

        let response = grid.handle(command, interface_state.save_data.clone());

        interface_state.messages.push(response.message);
        interface_state.save_data = response.save_data;

        // TODO: REPLACE WITH ACTUAL UI
        if response.aux_data.is_some() {
            interface_state.aux_data = response.aux_data;
        }

        interface_state.readout_scroll.scroll_to_bottom();
    }
}
