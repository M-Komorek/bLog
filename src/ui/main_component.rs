use ratatui::widgets::List;
use ratatui::Frame;

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let items = app.log_view.get_current_page_logs();
    let log_list = List::new(items);

    frame.render_widget(log_list, frame.size())
}
