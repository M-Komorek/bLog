use blog::app::{App, AppResult};
use blog::framework::{event_proxy, handlers, ArgParser, EventProxy, TuiManager};

use clap::Parser;
use log4rs;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

fn main() -> AppResult<()> {
    log4rs::init_file("log4rs.yaml", Default::default())?;

    let arg_parser = ArgParser::parse();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let terminal_height = terminal.size()?.height as usize;

    let event_proxy = EventProxy::new(250);
    let mut tui = TuiManager::new(terminal, event_proxy);
    tui.init()?;

    let mut app = App::new(arg_parser.log_file_path(), terminal_height)?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.event_proxy.next_event()? {
            event_proxy::Event::Tick => app.tick(),
            event_proxy::Event::Key(key_event) => handlers::handle_key_events(key_event, &mut app)?,
            event_proxy::Event::Resize(_, rows) => handlers::handle_resize_event(rows, &mut app)?,
        }
    }

    tui.exit()?;
    Ok(())
}
