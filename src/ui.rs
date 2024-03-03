use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!(
            "There will be logs.\n\
                Press `Esc`  or `q` to stop running."
        ))
        .block(Block::bordered().title(" -  bLog  - "))
        .centered(),
        frame.size(),
    )
}
