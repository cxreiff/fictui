use bevy::prelude::*;
use ratatui::{
    buffer::Buffer,
    prelude::Rect,
    text::Text,
    widgets::{self, Block, Borders, Padding, Widget},
};

use super::prompt::PromptSubmitted;

#[derive(Resource, Default)]
pub struct Readout {
    pub messages: Vec<String>,
}

pub fn readout_plugin(app: &mut App) {
    app.init_resource::<Readout>()
        .add_systems(Update, handle_prompt_submissions_system);
}

fn handle_prompt_submissions_system(
    mut prompt_submitted: EventReader<PromptSubmitted>,
    mut readout: ResMut<Readout>,
) {
    for submission in prompt_submitted.read() {
        readout.messages.push(submission.0.clone());
    }
}

impl Widget for &Readout {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1));

        let messages = self
            .messages
            .iter()
            .map(|msg| Text::from(msg.clone()))
            .collect::<widgets::List>()
            .block(block);

        messages.render(area, buf);
    }
}
