use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

pub struct LogViewer {}

impl Widget for LogViewer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_string(
            area.left(),
            area.top(),
            "LogViewer",
            Style::default().fg(Color::Green),
        );
    }
}
