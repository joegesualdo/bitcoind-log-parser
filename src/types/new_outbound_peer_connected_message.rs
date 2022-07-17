use core::fmt;

use crate::utilities::{get_key_value_from_key_value_string, remove_trailing_comma};

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

#[derive(Debug)]
pub struct NewOutboundPeerConnectedMessage {
    pub version: u64,
    pub blocks: u64,
    pub peer: u64,
    pub peeraddr: String,
    pub connection_type: OutboundConnection,
    pub raw: String,
}

impl NewOutboundPeerConnectedMessage {
    pub fn is_new_outbound_peer_log_line(message: &String) -> bool {
        return message.starts_with("New outbound peer connected:");
    }

    pub fn parse(message: &String) -> Option<NewOutboundPeerConnectedMessage> {
        if !Self::is_new_outbound_peer_log_line(message) {
            return None;
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
        return Some(NewOutboundPeerConnectedMessage {
            version,
            blocks,
            peer,
            peeraddr,
            connection_type,
            raw: message.clone(),
        });
    }
}
