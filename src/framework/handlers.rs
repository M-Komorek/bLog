use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('j') => {
            app.log_view.next_page();
        }
        KeyCode::Char('k') => {
            app.log_view.prev_page();
        }
        KeyCode::Char('q') => {
            app.quit();
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_handle_key_events_for_quit() {
        let mut temp_file = NamedTempFile::new().expect("should create NamedTempFile");
        write!(temp_file, "Line of logs").expect("should write a line of logs");
        let mut app = App::new(&temp_file.path().to_path_buf()).expect("should create App");

        let key_event_q = KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: event::KeyModifiers::empty(),
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::empty(),
        };

        handle_key_events(key_event_q, &mut app).expect("should handle q key event");
        assert!(app.running == false);
    }
}
