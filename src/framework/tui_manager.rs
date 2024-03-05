use super::event_proxy::EventProxy;
use crate::app::{App, AppResult};
use crate::ui;

use crossterm::event::DisableMouseCapture;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io;
use std::panic;

#[derive(Debug)]
pub struct TuiManager<B: Backend> {
    pub event_proxy: EventProxy,
    terminal: Terminal<B>,
}

impl<B: Backend> TuiManager<B> {
    pub fn new(terminal: Terminal<B>, event_proxy: EventProxy) -> Self {
        Self {
            event_proxy,
            terminal,
        }
    }

    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, DisableMouseCapture)?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("Failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal
            .draw(|frame| ui::main_component::render(app, frame))?;
        Ok(())
    }

    pub fn exit(&mut self) -> AppResult<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    fn reset() -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen)?;
        Ok(())
    }
}
