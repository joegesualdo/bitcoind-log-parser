// Helpful source: https://towardsdatascience.com/understand-your-comp&uter-system-using-logs-98139d0b5de1
use bitcoind_log_parser;
use std::{collections::HashMap, fs};
use bitcoind_log_parser::types::*;

fn main() {
    let file_contents =
        fs::read_to_string("../../../Desktop/bitcoind-log.txt").expect("failed to open file");
    let lines: Vec<&str> = file_contents.lines().collect();

    for line in lines {
        let log_line = parse_line(line);
        println!("---------------");
        println!("datetimestamp: {}", log_line.datetimestamp);
        println!("process: {}", log_line.process);
        match log_line.message_container.message {
            BitcoindLogMessage::NewOutboundPeerConnected(nopc) => {
                let NewOutboundPeerConnectedMessage {
                    version,
                    blocks,
                    peer,
                    peeraddr,
                    connection_type,
                    raw,
                } = nopc;
                println!("NEW PEER!");
                println!("version: {}", version);
                println!("blocks: {}", blocks);
                println!("peer: {}", peer);
                println!("peeraddr: {}", peeraddr);
                println!("connection_type: {:?}", connection_type);
                println!("raw: {}", raw);
            }
            BitcoindLogMessage::Unknown { raw } => {
                println!("UNKNOWN");
                println!("raw: {}", raw);
            }
            _ => {
                panic!("DONT KNOW THIS KIND")
            }
        }

        //println!("---------------");
    }

    fn parse_line(log_line: &str) -> BitcoindLogLine {
        let log_line = LogLine::parse_into_log_line(log_line);

        let log_message_seperated_by_spaces: Vec<&str> = log_line.message.split(" ").collect();
        let first_item = log_message_seperated_by_spaces[0].to_string();
        let does_first_item_end_with_colon: bool = first_item.chars().last().unwrap() == ':';
        let category = if does_first_item_end_with_colon {
            let first_item_without_colon: String =
                first_item[0..(first_item.len() - 1)].to_string();
            Category::get_category_type(&first_item_without_colon)
        } else {
            None
        };

        let bitcoind_log_message =
            if NewOutboundPeerConnectedMessage::is_new_outbound_peer_log_line(&log_line.message) {
                NewOutboundPeerConnectedMessage::parse(&log_line.message)
                    .unwrap_or_else(|| panic!("Ut oh"))
            } else {
                BitcoindLogMessage::Unknown {
                    raw: log_line.message,
                }
            };

        let bitcoind_log_line = BitcoindLogLine {
            datetimestamp: log_line.header.datetimestamp,
            process: log_line.header.process,
            message_container: BitcoindLogMessageContainer {
                message: bitcoind_log_message,
                category,
            },
        };
        return bitcoind_log_line;
    }
    // let first_line = lines.get(0).unwrap().clone();
    // let rest_of_lines = &lines[0..];

    // println!("first line: {}", lines[0]);
    // println!("rest of lines: {}", rest_of_lines[2..].join(" "));
}
