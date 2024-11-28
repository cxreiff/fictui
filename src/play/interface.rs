use std::io;

use bevy::prelude::*;
use bevy::utils::error;
use bevy_ratatui::terminal::RatatuiContext;
use ratatui::layout::{Constraint, Layout};
use ratatui::layout::{Position, Rect};
use ratatui::text::{Line, Text};
use ratatui::widgets::{self, Block, Borders, Padding, Paragraph};
use ratatui::Frame;
use tui_input::Input;

pub fn interface_plugin(app: &mut App) {
    app.add_systems(Update, draw_scene_system.map(error))
        .init_resource::<InterfaceState>();
}

fn draw_scene_system(
    mut ratatui: ResMut<RatatuiContext>,
    interface_state: Res<InterfaceState>,
) -> io::Result<()> {
    ratatui.draw(|frame| interface_state.draw(frame, frame.area()))?;
    Ok(())
}

#[derive(Resource, Default)]
pub struct InterfaceState {
    pub _focus: InterfaceFocus,
    pub prompt_input: Input,
    pub messages: Vec<String>,
}

#[derive(Default)]
pub enum InterfaceFocus {
    #[default]
    Prompt,
}

impl InterfaceState {
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let constraints = [Constraint::Min(1), Constraint::Length(3)];
        let layout = Layout::vertical(constraints).split(area);

        self.draw_readout(frame, layout[0]);
        self.draw_prompt(frame, layout[1]);
    }

    fn draw_readout(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1));

        let messages = self
            .messages
            .iter()
            .map(|msg| Text::from(msg.clone()))
            .collect::<widgets::List>()
            .block(block);

        frame.render_widget(messages, area);
    }

    fn draw_prompt(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::ALL);

        let constraints = [Constraint::Length(3), Constraint::Min(3)];
        let layout = Layout::horizontal(constraints).split(block.inner(area));

        self.draw_icon(frame, layout[0]);
        self.draw_input(frame, layout[1]);
        frame.render_widget(block, area);
    }

    fn draw_icon(&self, frame: &mut Frame, area: Rect) {
        let icon = Line::from(" > ");
        frame.render_widget(icon, area);
    }

    fn draw_input(&self, frame: &mut Frame, area: Rect) {
        let scroll = self.prompt_input.visual_scroll(area.width as usize - 1);
        let input = Paragraph::new(self.prompt_input.value()).scroll((0, scroll as u16));

        self.update_cursor(frame, area, scroll);
        frame.render_widget(input, area);
    }

    fn update_cursor(&self, frame: &mut Frame, area: Rect, scroll: usize) {
        let cursor_offset = (self.prompt_input.visual_cursor().max(scroll) - scroll) as u16;
        frame.set_cursor_position(Position::new(area.x + cursor_offset, area.y));
    }
}
