use clap::Parser;
use log::debug;
use log4rs;

mod arg_parser;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default())
        .expect("Failed to load log4rs configuration");

    let arg_parser = arg_parser::ArgParser::parse();
    debug!(
        "Provided file path of log file: {}",
        arg_parser.log_file_path().to_str().unwrap()
    );
}
