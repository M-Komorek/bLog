use blog::app::{App, AppResult};
use blog::arg_parser::ArgParser;
use blog::event_handler::{handle_key_events, Event, EventHandler};
use blog::tui::Tui;
use clap::Parser;
use log4rs;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

fn main() -> AppResult<()> {
    log4rs::init_file("log4rs.yaml", Default::default())?;

    let arg_parser = ArgParser::parse();
    let mut app = App::new(arg_parser.log_file_path())?;

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.event_handler.next_event()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
        }
    }

    tui.exit()?;
    Ok(())
}
