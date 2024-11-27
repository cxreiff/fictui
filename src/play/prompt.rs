use bevy::prelude::*;
use bevy_ratatui::event::KeyEvent;
use ratatui::{buffer::Buffer, crossterm::event::{Event, KeyCode, KeyEventKind}, prelude::Rect, widgets::{Block, Borders, Padding, Paragraph, Widget}};
use tui_input::{backend::crossterm::EventHandler, Input};

#[derive(Resource, Default)]
pub struct Prompt {
    pub input: Input,
}

#[derive(Event, Deref, DerefMut)]
pub struct PromptSubmitted(pub String);

pub fn prompt_plugin(app: &mut App) {
    app
        .init_resource::<Prompt>()
        .add_event::<PromptSubmitted>()
        .add_systems(PreUpdate, handle_keyboard_system);
}

fn handle_keyboard_system(mut ratatui_events: EventReader<KeyEvent>, mut prompt_submitted: EventWriter<PromptSubmitted>, mut prompt: ResMut<Prompt>) {
    for key_event in ratatui_events.read() {
        match key_event.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => match key_event.code {
                KeyCode::Enter => {
                    prompt_submitted.send(PromptSubmitted(prompt.input.value().into()));
                    prompt.input.reset();
                }
                KeyCode::Esc => {
                    prompt.input.reset();
                }
                _ => {
                    prompt.input.handle_event(&Event::Key(key_event.0));
                }
            },
            _ => {}
        }
    }
}

impl Widget for &Prompt {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let scroll = self.input.visual_scroll(area.width as usize);
        let block = Block::default().borders(Borders::ALL).padding(Padding::uniform(1)).title("PROMPT");
        let input = Paragraph::new(self.input.value())
            .scroll((0, scroll as u16))
            .block(block);

        input.render(area, buf);
    }
}
