mod log_header;
mod log_message;

pub use log_header::LogHeader;
pub use log_message::LogMessage;

use crate::ParseError;

#[derive(Debug)]
pub struct LogLine {
    pub header: LogHeader,
    pub message: LogMessage,
}

impl LogLine {
    // TODO: Refactor with less nesting.
    pub fn parse(log_line_string: &str) -> Result<LogLine, ParseError> {
        let log_header_result = LogHeader::parse(log_line_string);

        match log_header_result {
            Ok((log_header, raw_log_message)) => {
                let log_message_result = LogMessage::parse(raw_log_message);
                match log_message_result {
                    Ok(log_message) => Ok(LogLine {
                        header: log_header,
                        message: log_message,
                    }),
                    Err(_) => Err(ParseError),
                }
            }
            Err(_) => Err(ParseError),
        }
    }
}
