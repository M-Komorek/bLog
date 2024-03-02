use clap::Parser;
use log::{debug, error};
use log4rs;

use crate::logs::RawLogs;

mod arg_parser;
mod logs;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default())
        .expect("Failed to load log4rs configuration");

    let arg_parser = arg_parser::ArgParser::parse();
    debug!(
        "Providedlog file path: {}",
        arg_parser.log_file_path().to_str().unwrap()
    );

    match RawLogs::from_file(arg_parser.log_file_path()) {
        Ok(raw_logs) => debug!("RawLogs abstraction created"),
        Err(err) => error!("RawLogs creation failed. Error message: {}", err),
    }
}
