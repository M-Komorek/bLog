use std::path::PathBuf;

use super::logs::LogView;
use super::AppResult;

pub struct App {
    pub running: bool,
    pub log_view: LogView,
}

impl App {
    pub fn new(log_file_path: &PathBuf) -> AppResult<App> {
        let log_view = LogView::from_file(log_file_path)?;
        Ok(App {
            running: true,
            log_view,
        })
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }
}
