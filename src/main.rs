use clap::Parser;

mod arg_parser;

fn main() {
    let arg_parser = arg_parser::ArgParser::parse();
    println!(
        "log filepath: {}",
        arg_parser.log_file_path().to_str().unwrap()
    );
}
