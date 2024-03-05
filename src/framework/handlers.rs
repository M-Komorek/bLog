use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    //TODO: write tests for test_handle_key_events after inytoduce of IMP
    /*    #[test]
    fn test_handle_key_events_quit() {
        let mut app = App::new();
        let key_event_q = KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: event::KeyModifiers::empty(),
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::empty(),
        };
        let key_event_esc = KeyEvent {
            code: KeyCode::Esc,
            modifiers: event::KeyModifiers::empty(),
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::empty(),
        };

        handle_key_events(key_event_q, &mut app).unwrap();
        assert!(app.is_quitting());
    }
    */
}
