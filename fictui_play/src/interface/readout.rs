use ratatui::{
    layout::{Rect, Size},
    style::{Color, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};
use tui_scrollview::{ScrollView, ScrollbarVisibility};

use super::InterfaceState;

impl InterfaceState {
    pub(super) fn draw_readout(&mut self, frame: &mut Frame, area: Rect) {
        // Create the containing block.
        let block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1));
        let inner_area = block.inner(area);

        // Interleave commands and messages into a paragraph.
        let paragraph = Paragraph::new(
            self.commands
                .iter()
                .zip(self.messages.iter())
                .flat_map(|(cmd, msg)| {
                    let cmd_line = vec![Line::from(vec![
                        Span::from(">\u{00a0}").fg(Color::Magenta),
                        Span::from(cmd).fg(Color::Cyan),
                    ])];
                    let msg_lines = msg.lines().map(Line::from).collect();
                    let separator = vec!["".into()];
                    [cmd_line, separator.clone(), msg_lines, separator].concat()
                })
                .collect::<Vec<Line>>(),
        )
        .wrap(Wrap::default());

        // Determine the size of the scroll content and create the scroll view.
        let paragraph_size = Size::new(
            inner_area.width,
            paragraph.line_count(inner_area.width) as u16,
        );
        let mut scroll_view = ScrollView::new(paragraph_size);

        // We will add or subtract from this value if resizes would change how far we are scrolled
        // from the bottom of the scroll content, and then we will use it to correct the scroll
        // offset to be preserved relative to the bottom instead of top. We do this because the
        // default is to be scrolled to the bottom where new content appears.
        let mut resize_scroll_shift = 0;

        // Handle the case where the height of the scrollable content changes.
        if paragraph_size.height != self.previous_content_size.height {
            resize_scroll_shift +=
                paragraph_size.height as i16 - self.previous_content_size.height as i16;
        }

        // Handle the case where the height of the scroll view changes.
        if inner_area.height != self.previous_scroll_view_size.height {
            resize_scroll_shift +=
                self.previous_scroll_view_size.height as i16 - inner_area.height as i16;
        }

        // Use the calculated shift to correct the offset to be constant relative to the bottom of
        // the scrolled content.
        let mut new_offset = self.readout_scroll.offset();
        new_offset.y = new_offset.y.saturating_add_signed(resize_scroll_shift);
        self.readout_scroll.set_offset(new_offset);

        // Update the previous sizes so that we can make the same scroll correction next frame.
        self.previous_content_size = paragraph_size;
        self.previous_scroll_view_size = Size::from(inner_area);

        // When scroll view is scrolled to bottom of content (the "default" state) hide the
        // scrollbars, even if there is scrollable content.
        if self.readout_scroll.offset().y
            > (paragraph_size.height).saturating_sub(inner_area.height)
        {
            scroll_view = scroll_view.scrollbars_visibility(ScrollbarVisibility::Never);
        }

        // Increase the render area for the scroll view by two so that the scrollbar overlaps with
        // the containing block's right border.
        let wider_inner_area = Rect {
            width: inner_area.width + 2,
            ..inner_area
        };

        // Render the block, scroll content, and scroll view.
        frame.render_widget(block, area);
        scroll_view.render_widget(paragraph, scroll_view.area());
        frame.render_stateful_widget(scroll_view, wider_inner_area, &mut self.readout_scroll);
    }
}
