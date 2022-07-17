use chrono::{DateTime, FixedOffset};

const SPACE: &str = " ";

type LogMessage = String;

fn strip_first_and_last(s: &str) -> &str{
    return &s[1..(s.len() - 1)]
}

fn is_surround_by_brackets(s: &str) -> bool {
    let first_char = s.chars().nth(0);
    let last_char = s.chars().nth(s.len()-1);
    let starts_with_paren = match first_char {
        Some(char) => char == '[',
        None => false
    };
    let ends_with_paren = match last_char {
        Some(char) => char == ']',
        None => false
    };
    return starts_with_paren && ends_with_paren
}

pub struct LogHeader {
    pub datetimestamp: DateTime<FixedOffset>,
    pub verbosity_level: Option<String>, // bitcoind doesn't provide this
    pub process: Option<String>,                 // bitcoind puts this inside brackets (i.e. [msghand])
}

pub struct LogLine {
    pub header: LogHeader,
    pub message: LogMessage,
    pub raw: String,
}

#[derive(Debug)]
pub struct ParseError;

impl LogLine {
    pub fn parse(line_string: &str) -> Result<LogLine, ParseError> {
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
                Some(strip_first_and_last(next_item_after_datetime).to_string())
            } else {
                None
            };

        let rfc3339 = DateTime::parse_from_rfc3339(&datetime_string);
        let datetime = match rfc3339 {
            Ok(dt) => dt,
            Err(_) => {return Err(ParseError)}
        };
        

        let log_header: LogHeader = LogHeader {
            datetimestamp: datetime,
            verbosity_level: None,
            process,
        };
        let start_of_message_index = if is_process_included { 2 } else {1};
        let log_message: LogMessage = log_line_seperated_by_spaces[start_of_message_index..]
            .join(SPACE)
            .trim()
            .to_string();
        let log_line: LogLine = LogLine {
            header: log_header,
            message: log_message,
            raw: line_string.to_string(),
        };
        Ok(log_line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_works() {
        let raw_bitcoind_log_line = 
            "2022-07-12T15:12:34Z [msghand] New outbound peer connected: version: 70015, blocks=744716, peer=12, peeraddr=143.110.238.132:8333 (outbound-full-relay)";
        let log_line = LogLine::parse(raw_bitcoind_log_line).unwrap();
        assert_eq!(log_line.header.datetimestamp.to_string(), "2022-07-12 15:12:34 +00:00");
        assert_eq!(log_line.header.verbosity_level, None);
        assert_eq!(log_line.header.process.unwrap(), "msghand");
        assert_eq!(log_line.message, "New outbound peer connected: version: 70015, blocks=744716, peer=12, peeraddr=143.110.238.132:8333 (outbound-full-relay)");
        assert_eq!(log_line.raw, raw_bitcoind_log_line);
    }
    #[test]
    fn parse_without_process() {
        let raw_bitcoind_log_line = 
            "2022-07-12T15:12:34Z New outbound peer connected: version: 70015, blocks=744716, peer=12, peeraddr=143.110.238.132:8333 (outbound-full-relay)";
        let log_line = LogLine::parse(raw_bitcoind_log_line).unwrap();
        assert_eq!(log_line.header.datetimestamp.to_string(), "2022-07-12 15:12:34 +00:00");
        assert_eq!(log_line.header.verbosity_level, None);
        assert_eq!(log_line.header.process, None);
        assert_eq!(log_line.message, "New outbound peer connected: version: 70015, blocks=744716, peer=12, peeraddr=143.110.238.132:8333 (outbound-full-relay)");
        assert_eq!(log_line.raw, raw_bitcoind_log_line);
    }
}
