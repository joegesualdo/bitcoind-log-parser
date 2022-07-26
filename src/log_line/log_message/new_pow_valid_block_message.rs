use std::io;

use crate::log_line::log_message::message::Message;
use crate::log_line::log_message::parse_error::ParseError;
use crate::log_line::log_message::parse_result::ParseResult;
use crate::utils::get_key_value_from_key_value_string;

#[derive(Debug)]
pub struct NewPoWValidBlockMessage {
    hash: String,
}

impl Message<Self> for NewPoWValidBlockMessage {
    fn is_valid(raw_message: &str) -> bool {
        let is_valid = raw_message.starts_with("NewPoWValidBlock:");
        is_valid
    }
    fn parse(raw_message: &str) -> ParseResult<Self> {
        if !Self::is_valid(raw_message) {
            return Err(ParseError);
        } else {
            let message_split_by_spaces: Vec<&str> = raw_message.split_whitespace().collect();
            let hash_key_value_str = message_split_by_spaces[2];
            let hash_key_value = get_key_value_from_key_value_string(hash_key_value_str);
            let hash_value = hash_key_value[1];
            Ok(Self {
                hash: hash_value.to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_npowvbm_works() {
        let raw_message = "NewPoWValidBlock: block hash=00000000000000000006778a7fb1adf22b4fe6ae58401ae1e61baa4394397ae0";
        let message_result = NewPoWValidBlockMessage::parse(&raw_message);
        let hash = message_result.unwrap().hash;
        assert_eq!(
            hash,
            "00000000000000000006778a7fb1adf22b4fe6ae58401ae1e61baa4394397ae0"
        );

        let raw_message = "NewPoWValidBlock: block hash=123456789";
        let message_result = NewPoWValidBlockMessage::parse(&raw_message);
        let hash = message_result.unwrap().hash;
        assert_eq!(hash, "123456789");

        let raw_message = "no valid";
        let message = NewPoWValidBlockMessage::parse(&raw_message);
        assert!(message.is_err());
    }

    #[test]
    fn is_valid_works() {
        let raw_message = "NewPoWValidBlock: block hash=00000000000000000006778a7fb1adf22b4fe6ae58401ae1e61baa4394397ae0";
        let is_valid = NewPoWValidBlockMessage::is_valid(&raw_message);
        assert_eq!(is_valid, true);

        let raw_message = "no valid";
        let is_valid = NewPoWValidBlockMessage::is_valid(&raw_message);
        assert_eq!(is_valid, false);
    }
}
