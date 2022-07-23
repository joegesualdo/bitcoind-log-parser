use crate::log_line::log_message::parse_result::ParseResult;

pub trait Message<T> {
    fn is_valid(message: &str) -> bool;
    fn parse(message: &str) -> ParseResult<T>;
}
