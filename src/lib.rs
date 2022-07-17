//! # Bitcoind Log Parser
//!
//! Parse Bitcoind logs
//!
pub mod types;
mod utilities;

use types::BitcoindLogLine;

pub fn parse(log_line: &str) -> BitcoindLogLine {
    return BitcoindLogLine::parse(log_line);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first_item_in_log_line_test() {}
}
