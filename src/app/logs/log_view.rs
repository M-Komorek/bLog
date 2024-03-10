use log::debug;
use ratatui::text::Text;
use std::io::Result;
use std::path::PathBuf;
use std::str;

use super::raw_logs::RawLogs;

pub struct LogView {
    raw_logs: RawLogs,
    current_page: usize,
    lines_per_page: usize,
}

impl LogView {
    pub fn from_file(log_file_path: &PathBuf) -> Result<LogView> {
        let raw_logs = RawLogs::from_file(log_file_path)?;

        Ok(LogView {
            raw_logs,
            current_page: 1,
            lines_per_page: 10,
        })
    }

    pub fn get_current_page(&self) -> Vec<Text> {
        let start_index = (self.current_page - 1) * self.lines_per_page;
        let end_index = std::cmp::min(self.raw_logs.len(), start_index + self.lines_per_page);
        debug!("start_index: {}, end_inder: {}", start_index, end_index);
        self.raw_logs.data()[start_index..end_index]
            .iter()
            .map(|log| {
                let text = str::from_utf8(log).unwrap();
                Text::from(text.to_owned())
            })
            .collect()
    }

    pub fn next_page(&mut self) {
        if (self.current_page * self.lines_per_page) < self.raw_logs.len() {
            self.current_page += 1;
        }
    }

    pub fn prev_page(&mut self) {
        if self.current_page > 1 {
            self.current_page -= 1;
        }
    }
}
