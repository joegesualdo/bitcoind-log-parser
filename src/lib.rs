//! # Bitcoind Log Parser
//!
//! Parse Bitcoind logs
//!
mod log_line;
mod utils;

pub use log_line::{LogLine, LogMessage};

#[derive(Debug)]
pub struct ParseError;

pub fn parse(log_line: &str) -> Result<LogLine, ParseError> {
    let log_line_result = LogLine::parse(log_line);
    match log_line_result {
        Ok(log_line) => Ok(log_line),
        Err(_) => Err(ParseError),
    }
}
