use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

use super::InterfaceState;

impl InterfaceState {
    pub(super) fn draw_layout(&mut self, frame: &mut Frame, area: Rect) {
        let constraints = [
            Constraint::Length(18),
            Constraint::Min(3),
            Constraint::Length(3),
        ];

        let layout = Layout::vertical(constraints).split(area);

        self.draw_map(frame, layout[0]);
        self.draw_readout(frame, layout[1]);
        self.draw_prompt(frame, layout[2]);
    }
}
