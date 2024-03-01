use clap::Parser;
use std::path::PathBuf;

/// Blazingly fast tool for viewing log files.
/// It makes other tools feel like they're stuck in slow-motion...
#[derive(Parser)]
#[command(author, version, about)]
pub struct ArgParser {
    /// Path to the log file
    #[arg(long, required = true)]
    log_file_path: PathBuf,
}

impl ArgParser {
    pub fn log_file_path(&self) -> &PathBuf {
        &self.log_file_path
    }
}
