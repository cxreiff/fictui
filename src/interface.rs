use std::io;

use bevy::prelude::*;
use bevy::utils::error;
use bevy_ratatui::terminal::RatatuiContext;
use ratatui::{
    symbols::border,
    text::Text,
    widgets::{Block, Padding, Paragraph},
};

pub fn interface_plugin(app: &mut App) {
    app.add_systems(Update, draw_scene_system.map(error));
}

fn draw_scene_system(mut ratatui: ResMut<RatatuiContext>) -> io::Result<()> {
    ratatui.draw(|frame| {
        let block = Block::bordered()
            .border_set(border::THICK)
            .padding(Padding::new(0, 0, frame.area().height / 2, 0));
        let paragraph = Paragraph::new(Text::from("fictui")).centered().block(block);
        frame.render_widget(paragraph, frame.area());
    })?;

    Ok(())
}
