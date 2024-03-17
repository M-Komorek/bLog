use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, Table};
use ratatui::Frame;
use style::palette::tailwind;

use crate::app::App;

const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;

pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());

    let items = app.log_view.get_current_page_logs();

    let outer_block = Block::default();

    let inner_block = Block::default()
        .borders(Borders::NONE)
        .fg(TEXT_COLOR)
        .bg(NORMAL_ROW_COLOR);

    let outer_area = layout[0];
    let inner_area = outer_block.inner(outer_area);

    outer_block.render(outer_area, frame.buffer_mut());

    let log_list = List::new(items)
        .block(inner_block)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::REVERSED)
                .fg(SELECTED_STYLE_FG),
        )
        .highlight_symbol(" >> ")
        .highlight_spacing(HighlightSpacing::Always);

    frame.render_stateful_widget(log_list, inner_area, &mut app.log_view.state.clone());

    let items2 = app.log_view.get_current_page_logs_rows();

    let outer_block2 = Block::default();

    let inner_block2 = Block::default()
        .borders(Borders::NONE)
        .fg(TEXT_COLOR)
        .bg(NORMAL_ROW_COLOR);

    let outer_area2 = layout[1];
    let inner_area2 = outer_block2.inner(outer_area2);

    outer_block2.render(outer_area, frame.buffer_mut());

    let table = Table::new(
        items2,
        [Constraint::Percentage(5), Constraint::Percentage(95)],
    )
    .block(inner_block2)
    .highlight_style(
        Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(SELECTED_STYLE_FG),
    );

    frame.render_stateful_widget(table, inner_area2, &mut app.log_view.table_state.clone());
}
