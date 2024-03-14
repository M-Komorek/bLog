use ratatui::{text::Text, widgets::ListState};
use std::str;

use super::raw_logs::RawLogs;

pub struct LogView {
    raw_logs: RawLogs,
    lines_per_page: usize,
    current_page: usize,
    pub state: ListState,
    pub horizontal_scroll: usize,
}

impl LogView {
    pub fn new(raw_logs: RawLogs, lines_per_page: usize) -> LogView {
        LogView {
            raw_logs,
            lines_per_page,
            current_page: 1,
            state: ListState::default(),
            horizontal_scroll: 0,
        }
    }

    pub fn get_current_page_logs(&self) -> Vec<Text> {
        let start_index = (self.current_page - 1) * self.lines_per_page;
        let end_index = std::cmp::min(self.raw_logs.len(), start_index + self.lines_per_page);

        self.raw_logs.data()[start_index..end_index]
            .iter()
            .map(|log| {
                let text = str::from_utf8(log).unwrap();
                if self.horizontal_scroll > text.len() {
                    Text::from("")
                } else {
                    Text::from(text[self.horizontal_scroll..].to_owned())
                }
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

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.lines_per_page - 1 {
                    self.next_page();
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.prev_page();
                    self.lines_per_page - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn set_lines_per_page(&mut self, lines_per_page: usize) {
        self.lines_per_page = lines_per_page;
    }
}
