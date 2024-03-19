use ratatui::widgets::{Row, TableState};

use super::raw_logs::RawLogs;

const NUMBER_OF_LOGS_PER_PAGE: usize = 100;

pub struct LogView {
    raw_logs: RawLogs,
    current_page_numer: usize,
    pub table_state: TableState,
    pub horizontal_scroll: usize,
}

impl LogView {
    pub fn new(raw_logs: RawLogs) -> LogView {
        let table_state = TableState::default().with_selected(Some(0));

        LogView {
            raw_logs,
            current_page_numer: 0,
            table_state,
            horizontal_scroll: 0,
        }
    }

    pub fn get_current_page_logs_rows(&self) -> Vec<Row> {
        let start_index = (self.current_page_numer) * NUMBER_OF_LOGS_PER_PAGE;
        let end_index = std::cmp::min(self.raw_logs.len(), start_index + NUMBER_OF_LOGS_PER_PAGE);

        let mut rows = Vec::with_capacity(NUMBER_OF_LOGS_PER_PAGE);
        for (id, log) in self.raw_logs.data()[start_index..end_index]
            .iter()
            .enumerate()
        {
            let text = if self.horizontal_scroll > log.len() {
                "".to_string()
            } else {
                String::from_utf8_lossy(&log[self.horizontal_scroll..]).to_string()
            };
            let index = (start_index + id).to_string();
            rows.push(Row::new(vec![index, text]));
        }

        rows
    }

    pub fn next_page(&mut self) {
        if (self.current_page_numer * NUMBER_OF_LOGS_PER_PAGE) < self.raw_logs.len() {
            self.current_page_numer += 1;
        }
    }

    pub fn prev_page(&mut self) {
        if self.current_page_numer > 0 {
            self.current_page_numer -= 1;
        }
    }

    pub fn next(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= NUMBER_OF_LOGS_PER_PAGE - 1 {
                    self.next_page();
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    if self.current_page_numer == 0 {
                        0
                    } else {
                        self.prev_page();
                        NUMBER_OF_LOGS_PER_PAGE - 1
                    }
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}
