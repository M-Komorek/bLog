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
    let mut app = App::new(arg_parser.log_file_path())?;

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let event_proxy = EventProxy::new(250);
    let mut tui = TuiManager::new(terminal, event_proxy);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.event_proxy.next_event()? {
            event_proxy::Event::Tick => app.tick(),
            event_proxy::Event::Key(key_event) => handlers::handle_key_events(key_event, &mut app)?,
        }
    }

    tui.exit()?;
    Ok(())
}
