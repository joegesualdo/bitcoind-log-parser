use crate::log_line::log_message::parse_error::ParseError;

pub type ParseResult<T> = Result<T, ParseError>;
