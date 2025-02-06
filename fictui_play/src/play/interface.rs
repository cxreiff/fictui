use std::io;

use bevy::prelude::*;
use bevy::utils::error;
use bevy_ratatui::terminal::RatatuiContext;
use fictui_core::grid::save_data::SaveData;
use ratatui::layout::{Constraint, Layout, Size};
use ratatui::layout::{Position, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};
use ratatui::Frame;
use tui_input::Input;
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

pub fn interface_plugin(app: &mut App) {
    app.add_systems(Update, draw_scene_system.map(error))
        .init_resource::<InterfaceState>();
}

fn draw_scene_system(
    mut ratatui: ResMut<RatatuiContext>,
    mut interface_state: ResMut<InterfaceState>,
) -> io::Result<()> {
    ratatui.draw(|frame| interface_state.draw(frame, frame.area()))?;
    Ok(())
}

#[derive(Resource, Default)]
pub struct InterfaceState {
    pub _focus: InterfaceFocus,
    pub prompt_input: Input,
    pub readout_scroll: ScrollViewState,
    previous_content_size: Size,
    previous_scroll_view_size: Size,
    pub commands: Vec<String>,
    pub messages: Vec<String>,
    pub save_data: SaveData,
}

#[derive(Default)]
pub enum InterfaceFocus {
    #[default]
    Prompt,
}

impl InterfaceState {
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let constraints = [Constraint::Min(3), Constraint::Length(3)];
        let layout = Layout::vertical(constraints).split(area);

        self.draw_readout(frame, layout[0]);
        self.draw_prompt(frame, layout[1]);
    }

    fn draw_readout(&mut self, frame: &mut Frame, area: Rect) {
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
                    vec![
                        Line::from(format!(">\u{00a0}{cmd}")).fg(Color::Rgb(220, 220, 220)),
                        Line::from(msg.clone()),
                        "".into(),
                    ]
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
