use crate::app::{App, AppResult};
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    Tick,
    Key(KeyEvent),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("Failed to poll new events") {
                        match event::read().expect("Unable to read event") {
                            CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                            _ => Ok(()),
                        }
                        .expect("Failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("Failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };

        EventHandler {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next_event(&self) -> AppResult<Event> {
        Ok(self.receiver.recv()?)
    }
}

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
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_event_handler_tick_event() {
        let tick_rate = 10;
        let event_handler = EventHandler::new(tick_rate);

        let start_time = Instant::now();
        let event = event_handler.next_event().unwrap();
        assert_eq!(event, Event::Tick);
        assert!(start_time.elapsed() >= Duration::from_millis(tick_rate));
    }

    #[test]
    fn test_event_handler_key_event() {
        let tick_rate = 10;
        let event_handler = EventHandler::new(tick_rate);

        let key_event = KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: event::KeyModifiers::empty(),
            kind: event::KeyEventKind::Press,
            state: event::KeyEventState::empty(),
        };

        event_handler.sender.send(Event::Key(key_event)).unwrap();
        let received_key_event = event_handler.next_event().unwrap();
        assert_eq!(received_key_event, Event::Key(key_event));
    }

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
