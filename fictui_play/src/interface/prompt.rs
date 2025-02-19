use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};

use super::InterfaceState;

impl InterfaceState {
    pub(super) fn draw_prompt(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::ALL);

        let constraints = [Constraint::Length(3), Constraint::Min(3)];
        let layout = Layout::horizontal(constraints).split(block.inner(area));

        self.draw_icon(frame, layout[0]);
        self.draw_input(frame, layout[1]);
        frame.render_widget(block, area);
    }
}
