use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Padding},
    Frame,
};

use super::{utils::center, InterfaceState};

impl InterfaceState {
    pub(super) fn draw_map(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1));
        let inner_area = block.inner(area);

        let inner_block = Block::default()
            .borders(Borders::all())
            .padding(Padding::horizontal(1))
            .fg(Color::Magenta);
        let center_area = center(inner_area, Constraint::Length(16), Constraint::Length(8));
        let neighbors = self
            .aux_data
            .as_ref()
            .map(|aux| aux.tile.neighbors.len())
            .unwrap_or(0);
        let text = Line::from(neighbors.to_string()).fg(Color::Cyan);

        let inner_center_area = inner_block.inner(center_area);

        frame.render_widget(block, area);
        frame.render_widget(inner_block, center_area);
        frame.render_widget(text, inner_center_area);
    }
}
