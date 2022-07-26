use chrono::{DateTime, FixedOffset};

const SPACE: &str = " ";

type LogMessage = String;

fn strip_first_and_last(s: &str) -> &str{
    return &s[1..(s.len() - 1)]
}

fn is_surround_by_brackets(s: &str) -> bool {
    const OPENENING_BRACKET: char = '[';
    const CLOSING_BRACKET: char = ']';
    let first_char = s.chars().nth(0);
    let last_char = s.chars().nth(s.len()-1);
    let starts_with_paren = match first_char {
        Some(char) => char == OPENENING_BRACKET,
        None => false
    };
    let ends_with_paren = match last_char {
        Some(char) => char == CLOSING_BRACKET,
        None => false
    };
    return starts_with_paren && ends_with_paren
}

#[derive(Debug)]
pub struct LogHeader {
    pub datetimestamp: DateTime<FixedOffset>,
    pub verbosity_level: Option<String>, // bitcoind doesn't provide this
    pub process: Option<String>,         // bitcoind puts this inside brackets (i.e. [msghand])
}


#[derive(Debug)]
pub struct ParseError;

impl LogHeader{
    // Takes a log line, parses it and returns a sturctured LogHeader and the raw message string
    pub fn parse(line_string: &str) -> Result<(Self, String), ParseError> {
        let log_line_seperated_by_spaces: Vec<&str> = 
            line_string
            .split_ascii_whitespace()
            .collect();
        let datetime_string: String = 
            log_line_seperated_by_spaces[0]
            .to_string();
        let next_item_after_datetime = log_line_seperated_by_spaces[1];
        let is_process_included = is_surround_by_brackets(next_item_after_datetime);
        let process: Option<String> = 
            if is_process_included && is_surround_by_brackets(next_item_after_datetime) {
                // TODO: Can probably use String.trim_matches() method to remove the brackets
                Some(strip_first_and_last(next_item_after_datetime).to_string())
            } else {
                None
            };

        let rfc3339 = DateTime::parse_from_rfc3339(&datetime_string);
        let datetime = match rfc3339 {
            Ok(dt) => dt,
            Err(_) => {return Err(ParseError)}
        };
        

        let log_header: Self = Self{
            datetimestamp: datetime,
            verbosity_level: None,
            process,
        };
        let start_of_message_index = if is_process_included { 2 } else {1};
        let log_message: LogMessage = log_line_seperated_by_spaces[start_of_message_index..]
            .join(SPACE)
            .trim()
            .to_string();
        let log_header: Self = Self{
            ..log_header
        };
        Ok((log_header, log_message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_surround_by_brackets_fn_works() {
        let surround_string = "[test]";
        let non_surrounded_string = "test";
        assert_eq!(is_surround_by_brackets(surround_string), true);
        assert_eq!(is_surround_by_brackets(non_surrounded_string), false);
    }
    #[test]
    fn strip_first_and_last_fn_works() {
        let s = "[test]";
        assert_eq!(strip_first_and_last(s), "test");
    }
    #[test]
    fn parse_fn_works() {
        let raw_bitcoind_log_line = 
            "2022-07-12T15:12:34Z [msghand] New outbound peer connected: version: 70015, blocks=744716, peer=12, peeraddr=143.110.238.132:8333 (outbound-full-relay)";
        let log_line = LogHeader::parse(raw_bitcoind_log_line).unwrap();
        assert_eq!(log_line.0.datetimestamp.to_string(), "2022-07-12 15:12:34 +00:00");
        assert_eq!(log_line.0.verbosity_level, None);
        assert_eq!(log_line.0.process.unwrap(), "msghand");
        assert_eq!(log_line.1, "New outbound peer connected: version: 70015, blocks=744716, peer=12, peeraddr=143.110.238.132:8333 (outbound-full-relay)");
    }
    #[test]
    fn parse_fn_without_process() {
        let raw_bitcoind_log_line = 
            "2022-07-12T15:12:34Z New outbound peer connected: version: 70015, blocks=744716, peer=12, peeraddr=143.110.238.132:8333 (outbound-full-relay)";
        let log_line = LogHeader::parse(raw_bitcoind_log_line).unwrap();
        assert_eq!(log_line.0.datetimestamp.to_string(), "2022-07-12 15:12:34 +00:00");
        assert_eq!(log_line.0.verbosity_level, None);
        assert_eq!(log_line.0.process, None);
        assert_eq!(log_line.1, "New outbound peer connected: version: 70015, blocks=744716, peer=12, peeraddr=143.110.238.132:8333 (outbound-full-relay)");
    }
}
