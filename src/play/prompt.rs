use bevy::prelude::*;
use bevy_ratatui::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    prelude::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};
use tui_input::{backend::crossterm::EventHandler, Input};

use super::interface::FrameState;

#[derive(Resource, Default)]
pub struct Prompt {
    pub input: Input,
}

#[derive(Event, Deref, DerefMut)]
pub struct PromptSubmitted(pub String);

pub fn prompt_plugin(app: &mut App) {
    app.init_resource::<Prompt>()
        .add_event::<PromptSubmitted>()
        .add_systems(PreUpdate, handle_keyboard_system);
}

fn handle_keyboard_system(
    mut ratatui_events: EventReader<KeyEvent>,
    mut prompt_submitted: EventWriter<PromptSubmitted>,
    mut prompt: ResMut<Prompt>,
) {
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
                    let mut new_event = key_event.0;
                    new_event.kind = KeyEventKind::Press;
                    prompt.input.handle_event(&Event::Key(new_event));
                }
            },
            _ => {}
        }
    }
}

impl StatefulWidget for &Prompt {
    type State = FrameState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default().borders(Borders::ALL);

        let constraints = [Constraint::Length(3), Constraint::Min(3)];
        let layout = Layout::horizontal(constraints).split(block.inner(area));

        let chevron = Line::from(" > ");

        let scroll = self.input.visual_scroll(layout[1].width as usize - 1);
        let input = Paragraph::new(self.input.value()).scroll((0, scroll as u16));

        let cursor_offset = (self.input.visual_cursor().max(scroll) - scroll) as u16;
        state.cursor_hint = (layout[1].x + cursor_offset, layout[1].y);

        block.render(area, buf);
        chevron.render(layout[0], buf);
        input.render(layout[1], buf);
    }
}
