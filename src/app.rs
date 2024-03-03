use std::path::PathBuf;

use crate::logs::RawLogs;

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct App {
    pub running: bool,
    pub raw_logs: RawLogs,
}

impl App {
    pub fn new(log_file_path: &PathBuf) -> AppResult<App> {
        let raw_logs = RawLogs::from_file(log_file_path)?;
        Ok(App {
            running: true,
            raw_logs,
        })
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }
}
