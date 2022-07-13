pub struct LogHeader {
    pub datetimestamp: String,
    pub verbosity_level: Option<String>, // bitcoind doesn't provide this
    pub process: String,                 // bitcoind puts this inside brackets (i.e. [msghand])
}

type LogMessage = String;

pub struct LogLine {
    pub header: LogHeader,
    pub message: LogMessage,
    pub raw: String,
}
impl LogLine {
    pub fn parse_into_log_line(line_string: &str) -> LogLine {
        let log_line_seperated_by_spaces: Vec<&str> = line_string.split(" ").collect();
        let datetime = log_line_seperated_by_spaces[0].to_string();
        let process_in_brackets = log_line_seperated_by_spaces[1];
        let process: String = process_in_brackets[1..(process_in_brackets.len() - 1)].to_string();
        let log_header: LogHeader = LogHeader {
            datetimestamp: datetime,
            verbosity_level: None,
            process,
        };
        let log_message: LogMessage = log_line_seperated_by_spaces[2..]
            .join(" ")
            .trim()
            .to_string();
        let log_line: LogLine = LogLine {
            header: log_header,
            message: log_message,
            raw: line_string.to_string(),
        };
        return log_line;
    }
}

