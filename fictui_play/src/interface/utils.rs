use ratatui::{
    layout::{Constraint, Flex, Layout, Position, Rect},
    text::Line,
    widgets::Paragraph,
    Frame,
};

use super::InterfaceState;

impl InterfaceState {
    pub(super) fn draw_icon(&self, frame: &mut Frame, area: Rect) {
        let icon = Line::from(" > ");
        frame.render_widget(icon, area);
    }

    pub(super) fn draw_input(&self, frame: &mut Frame, area: Rect) {
        let scroll = self.prompt_input.visual_scroll(area.width as usize - 1);
        let input = Paragraph::new(self.prompt_input.value()).scroll((0, scroll as u16));

        self.update_cursor(frame, area, scroll);
        frame.render_widget(input, area);
    }

    pub(super) fn update_cursor(&self, frame: &mut Frame, area: Rect, scroll: usize) {
        let cursor_offset = (self.prompt_input.visual_cursor().max(scroll) - scroll) as u16;
        frame.set_cursor_position(Position::new(area.x + cursor_offset, area.y));
    }
}

pub(super) fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
