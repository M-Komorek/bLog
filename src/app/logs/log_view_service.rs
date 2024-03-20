use ratatui::widgets::Row;

use super::log_view::LogViewState;
use super::raw_logs::RawLogs;

const NUMBER_OF_LOGS_PER_PAGE: usize = 100;

pub struct LogViewService {
    raw_logs: RawLogs,
    current_page_numer: usize,
    pub log_view_state: LogViewState,
    pub horizontal_offset: usize,
}

impl LogViewService {
    pub fn new(raw_logs: RawLogs) -> LogViewService {
        let table_state = LogViewState::default().with_selected(Some(0));

        LogViewService {
            raw_logs,
            current_page_numer: 0,
            log_view_state: table_state,
            horizontal_offset: 0,
        }
    }

    pub fn next_log(&mut self) {
        let i = match self.log_view_state.selected() {
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
        self.log_view_state.select(Some(i));
    }

    pub fn previous_log(&mut self) {
        let i = match self.log_view_state.selected() {
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
        self.log_view_state.select(Some(i));
    }

    pub fn get_current_page_logs_rows(&self) -> Vec<Row> {
        let start_index = (self.current_page_numer) * NUMBER_OF_LOGS_PER_PAGE;
        let end_index = std::cmp::min(self.raw_logs.len(), start_index + NUMBER_OF_LOGS_PER_PAGE);

        let mut rows = Vec::with_capacity(NUMBER_OF_LOGS_PER_PAGE);
        for (id, log) in self.raw_logs.data()[start_index..end_index]
            .iter()
            .enumerate()
        {
            let text = if self.horizontal_offset > log.len() {
                "".to_string()
            } else {
                String::from_utf8_lossy(&log[self.horizontal_offset..]).to_string()
            };
            let index = (start_index + id).to_string();

            rows.push(Row::new(vec![index, text]));
        }

        rows
    }

    fn next_page(&mut self) {
        if (self.current_page_numer * NUMBER_OF_LOGS_PER_PAGE) < self.raw_logs.len() {
            self.current_page_numer += 1;
        }
    }

    fn prev_page(&mut self) {
        if self.current_page_numer > 0 {
            self.current_page_numer -= 1;
        }
    }
}
