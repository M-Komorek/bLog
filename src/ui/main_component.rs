use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;
use style::palette::tailwind;

use crate::app::App;
use crate::app::LogView;

const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const TEXT_COLOR: Color = tailwind::SLATE.c200;

pub fn render(app: &mut App, frame: &mut Frame) {
    let items = app.log_view_service.get_current_page_logs_rows();

    let outer_block = Block::default();

    let inner_block = Block::default()
        .borders(Borders::NONE)
        .fg(TEXT_COLOR)
        .bg(NORMAL_ROW_COLOR);

    let outer_area = frame.size();
    let inner_area = outer_block.inner(outer_area);

    outer_block.render(outer_area, frame.buffer_mut());

    let log_view = LogView::new(
        items,
        [Constraint::Percentage(5), Constraint::Percentage(95)],
    )
    .block(inner_block)
    .highlight_style(Style::default().fg(Color::Yellow));

    frame.render_stateful_widget(
        log_view,
        inner_area,
        &mut app.log_view_service.log_view_state.clone(),
    );
}
