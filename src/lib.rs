//! # Bitcoind Log Parser
//!
//! Parse Bitcoind logs
//!
mod utilities;
pub mod types;

// pub fn parse_log_line(log_line: &str) -> &str {
    //parse a line and return the appropriate BitcoindLogMessage
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first_item_in_log_line_test() {
    }
}
