//! # Bitcoind Log Parser
//!
//! Parse Bitcoind logs
//!
mod utilities;
pub mod types;

use types::BitcoindLogLine;

pub fn parse_log_line(log_line: &str) -> BitcoindLogLine {
    return BitcoindLogLine::parse(log_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first_item_in_log_line_test() {
    }
}
