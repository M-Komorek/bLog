use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, HighlightSpacing, List};
use ratatui::Frame;
use style::palette::tailwind;

use crate::app::App;

const NORMAL_ROW_COLOR: Color = tailwind::SLATE.c950;
const SELECTED_STYLE_FG: Color = tailwind::BLUE.c300;
const TEXT_COLOR: Color = tailwind::SLATE.c200;

pub fn render(app: &mut App, frame: &mut Frame) {
    let items = app.log_view.get_current_page_logs();

    let outer_block = Block::default().borders(Borders::all());

    let inner_block = Block::default()
        .borders(Borders::NONE)
        .fg(TEXT_COLOR)
        .bg(NORMAL_ROW_COLOR);

    let outer_area = frame.size();
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
}
