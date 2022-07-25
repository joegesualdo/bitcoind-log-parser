use core::fmt;

use crate::log_line::log_message::message::Message;
use crate::log_line::log_message::parse_error::ParseError;
use crate::log_line::log_message::parse_result::ParseResult;

use crate::utils::{get_key_value_from_key_value_string, remove_trailing_comma};

#[derive(Debug)]
pub struct NewOutboundPeerConnectedMessage {
    pub version: u64,
    pub blocks: u64,
    pub peer: u64,
    pub peeraddr: String,
    pub connection_type: OutboundConnection,
    pub raw: String,
}

impl Message<NewOutboundPeerConnectedMessage> for NewOutboundPeerConnectedMessage {
    fn is_valid(message: &str) -> bool {
        return message.starts_with("New outbound peer connected:");
    }

    fn parse(message: &str) -> ParseResult<NewOutboundPeerConnectedMessage> {
        if !Self::is_valid(message) {
            return Err(ParseError);
        }

        let message_seperated_by_spaces: Vec<&str> = message.split(" ").collect();
        let version = message_seperated_by_spaces[5]
            .strip_suffix(',')
            .unwrap()
            .to_string()
            .parse()
            .expect("fail: 1");
        let message_seperated_by_spaces_after_version = message_seperated_by_spaces[6..].to_vec();
        // TODO: replace with .map so we don't need these mutable variables
        let mut blocks: u64 = 0;
        let mut peer: u64 = 0;
        let mut peeraddr: String = String::from("");
        // TODO: WRONG. Shouldn't set to an abritrary variant
        let mut connection_type: OutboundConnection = OutboundConnection::BlockRelayOnly;
        for part in message_seperated_by_spaces_after_version {
            if part.starts_with("blocks=") {
                let key_and_value = get_key_value_from_key_value_string(part);
                blocks = remove_trailing_comma(key_and_value[1])
                    .to_string()
                    .parse()
                    .unwrap();
            } else if part.starts_with("peer=") {
                let key_and_value = get_key_value_from_key_value_string(part);
                peer = remove_trailing_comma(key_and_value[1])
                    .to_string()
                    .parse()
                    .unwrap();
            } else if part.starts_with("peeraddr=") {
                let key_and_value = get_key_value_from_key_value_string(part);
                peeraddr = key_and_value[1].to_string();
            } else if part.starts_with("(") {
                connection_type = OutboundConnection::parse(part);
            }
        }
        return Ok(NewOutboundPeerConnectedMessage {
            version,
            blocks,
            peer,
            peeraddr,
            connection_type,
            raw: message.to_string(),
        });
    }
}

const OUTBOUND_FULL_RELAY_STR: &str = "outbound-full-relay";
const BLOCK_RELAY_ONLY_STR: &str = "block-relay-only";

// https://github.com/bitcoin/bitcoin/blob/87d012324afa285221073540781295f1b7381a15/src/net_processing.cpp#L2992
#[derive(Debug, PartialEq)]
pub enum OutboundConnection {
    OutboundFullRelay,
    BlockRelayOnly,
}

impl OutboundConnection {
    /// Takes the outbound connection part and converts it into an OutboundConnection type
    ///
    /// ```
    /// ```
    pub fn parse(string: &str) -> OutboundConnection {
        let connection_type_without_first_paren = string.strip_prefix('(').unwrap();
        let connection_type_without_last_paren = connection_type_without_first_paren
            .strip_suffix(')')
            .unwrap();
        let connection_type = connection_type_without_last_paren;
        let outbound_connection = match connection_type {
            OUTBOUND_FULL_RELAY_STR => OutboundConnection::OutboundFullRelay,
            BLOCK_RELAY_ONLY_STR => OutboundConnection::BlockRelayOnly,
            _ => panic!("Outbound connection type not found: {}", connection_type),
        };
        outbound_connection
    }
}

impl fmt::Display for OutboundConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OutboundConnection::BlockRelayOnly => OUTBOUND_FULL_RELAY_STR,
            OutboundConnection::OutboundFullRelay => BLOCK_RELAY_ONLY_STR,
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_works() {
        let valid_nopc_message = "New outbound peer connected: version: 70016, blocks=744842, peer=304, peeraddr=47.149.52.113:8333 (block-relay-only)";
        let is_valid = NewOutboundPeerConnectedMessage::is_valid(valid_nopc_message);
        assert_eq!(is_valid, true);

        let not_valid_nopc_message = "test";
        let is_valid = NewOutboundPeerConnectedMessage::is_valid(not_valid_nopc_message);
        assert_eq!(is_valid, false);
    }
    #[test]
    fn parse_works() {
        let valid_message_str = "New outbound peer connected: version: 70016, blocks=744842, peer=304, peeraddr=47.149.52.113:8333 (block-relay-only)";
        let parsed_result = NewOutboundPeerConnectedMessage::parse(valid_message_str);
        let message = parsed_result.unwrap();
        assert_eq!(message.version, 70016);
        assert_eq!(message.blocks, 744842);
        assert_eq!(message.peer, 304);
        assert_eq!(message.peeraddr, "47.149.52.113:8333");
        assert_eq!(message.connection_type, OutboundConnection::BlockRelayOnly);
        let invalid_test_cases = vec![
            "test",
            "test: version: 70016, blocks=744842, peer=304, peeraddr=47.149.52.113:8333 (block-relay-only)",
            "test: version: blocks=744842, peer=304, peeraddr=47.149.52.113:8333 (block-relay-only)",
            // TODO: Add more invalid cases
            // "New outbound peer connected: version: 70016, blocks=744842, peer=304, peeraddr=47.149.52.113:8333 (block-relay-only)";
        ];
        for invalid_test_case in invalid_test_cases {
            let parsed_result = NewOutboundPeerConnectedMessage::parse(invalid_test_case);
            let nopc_message = parsed_result;
            assert!(nopc_message.is_err());
        }
    }
}
