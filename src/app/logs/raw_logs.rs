use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::PathBuf;

type RawLog = Vec<u8>;

#[derive(Debug)]
pub struct RawLogs {
    data: Vec<RawLog>,
}

impl RawLogs {
    pub fn from_file(log_file_path: &PathBuf) -> Result<RawLogs> {
        let log_file = File::open(log_file_path)?;
        let reader = BufReader::new(log_file);

        let mut logs = Vec::new();
        for line_result in reader.split(b'\n') {
            let line = line_result?;
            logs.push(line);
        }

        Ok(RawLogs { data: logs })
    }

    pub fn data(&self) -> &[RawLog] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Write, str::FromStr};
    use tempfile::NamedTempFile;

    #[test]
    fn test_from_file_return_raw_logs_on_success() {
        let mut temp_file = NamedTempFile::new().expect("should create NamedTempFile");
        write!(temp_file, "Line 1\nLine 2\nLine 3").unwrap();

        let raw_logs_path = temp_file.path().to_path_buf();
        let raw_logs = RawLogs::from_file(&raw_logs_path).expect("should create RawLogs");

        assert_eq!(raw_logs.data[0], b"Line 1");
        assert_eq!(raw_logs.data[1], b"Line 2");
        assert_eq!(raw_logs.data[2], b"Line 3");
    }

    #[test]
    fn test_from_file_file_return_error_on_fail() {
        let invalid_path = PathBuf::from_str("invalid.log").expect("should create PathBuf");
        let raw_logs = RawLogs::from_file(&invalid_path);
        assert!(raw_logs.is_err());
    }
}
