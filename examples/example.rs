// Helpful source: https://towardsdatascience.com/understand-your-comp&uter-system-using-logs-98139d0b5de1
use bitcoind_log_parser;
use std::{fs, collections::HashMap};

enum BitcoindLogMessage {
    Unknown {
        raw: String,
    },
    NewOutboundPeerConnected {
        version: u64,
        blocks: u64,
        peer: u64,
        peeraddr: String,
        connection_type: String,
        raw: String
    }, // https://github.com/bitcoin/bitcoin/blob/87d012324afa285221073540781295f1b7381a15/src/net_processing.cpp#L2992
    UpdateTip, // https://github.com/bitcoin/bitcoin/blob/a7f3479ba3fda4c9fb29bd7080165744c02ee921/src/validation.cpp#L2504
    FlushStateToDisk, // https://github.com/bitcoin/bitcoin/blob/a7f3479ba3fda4c9fb29bd7080165744c02ee921/src/validation.cpp#L2426
    AssumingAncestorOfBlockHasValidSignatures, // https://github.com/bitcoin/bitcoin/blob/a7f3479ba3fda4c9fb29bd7080165744c02ee921/src/init.cpp#L917
    P2pPeersAvailableSkippedDnsSeeding, // https://github.com/bitcoin/bitcoin/blob/d571cf2d2421c6f8efb2b61ca844034eaf230945/src/net.cpp#L1445
    ThreadStart, // https://github.com/bitcoin/bitcoin/blob/2e30e328a7a46e0405664fd0cb31d971171f71d1/src/util/thread.cpp#L17
    ThreadExit, // https://github.com/bitcoin/bitcoin/blob/2e30e328a7a46e0405664fd0cb31d971171f71d1/src/util/thread.cpp#L19
    ImportedMempoolTransactionsFromDisk, // https://github.com/bitcoin/bitcoin/blob/a7f3479ba3fda4c9fb29bd7080165744c02ee921/src/validation.cpp#L4723
    InitMessage, // https://github.com/bitcoin/bitcoin/blob/38c63e3683746774d3ddc60e32aa33af20573473/src/noui.cpp#L56
    WaitingBeforeQueryingDnsSeeds, // https://github.com/bitcoin/bitcoin/blob/d571cf2d2421c6f8efb2b61ca844034eaf230945/src/net.cpp#L1423
    BlockRelayOnlyAnchorsWillBeTriedForConnections, // https://github.com/bitcoin/bitcoin/blob/d571cf2d2421c6f8efb2b61ca844034eaf230945/src/net.cpp#L2284
}

// Source: https://man.archlinux.org/man/community/bitcoin-daemon/bitcoind.1.en
//   see -debug section
enum Category {
    Addrman,
    Bench,
    Blockstorage,
    Cmpctblock,
    Coindb,
    Estimatefee,
    Http,
    I2p,
    Ipc,
    Leveldb,
    Libevent,
    Mempool,
    Mempoolrej,
    Net,
    Proxy,
    Prune,
    Qt,
    Rand,
    Reindex,
    Rpc,
    Selectcoins,
    Tor,
    Util,
    Validation,
    Walletdb,
    Zmq,
}

fn get_category_type(category: &str) -> Option<Category> {
    match category {
        "addrman" => Some(Category::Addrman),
        "bench" => Some(Category::Bench),
        "blockstorage" => Some(Category::Blockstorage),
        "cmpctblock" => Some(Category::Cmpctblock),
        "coindb" => Some(Category::Coindb),
        "estimatefee" => Some(Category::Estimatefee),
        "http" => Some(Category::Http),
        "i2p" => Some(Category::I2p),
        "ipc" => Some(Category::Ipc),
        "leveldb" => Some(Category::Leveldb),
        "libevent" => Some(Category::Libevent),
        "mempool" => Some(Category::Mempool),
        "mempoolrej" => Some(Category::Mempoolrej),
        "net" => Some(Category::Net),
        "proxy" => Some(Category::Proxy),
        "prune" => Some(Category::Prune),
        "qt" => Some(Category::Qt),
        "rand" => Some(Category::Rand),
        "reindex" => Some(Category::Reindex),
        "rpc" => Some(Category::Rpc),
        "selectcoins" => Some(Category::Selectcoins),
        "tor" => Some(Category::Tor),
        "util" => Some(Category::Util),
        "validation" => Some(Category::Validation),
        "walletdb" => Some(Category::Walletdb),
        "zmq" => Some(Category::Zmq),
        unrecognized_category => None,
    }
}

struct LogLineRaw {
    header: String,
    message: String,
}

struct LogHeader {
    datetimestamp: String,
    verbosity_level: Option<String>, // bitcoind doesn't provide this
    process: String, // bitcoind puts this inside brackets (i.e. [msghand])
}

type LogMessage = String;

struct LogLine {
    header: LogHeader,
    message: LogMessage,
    raw: String,
}

struct BitcoindLogMessageContainer {
    kind: BitcoindLogMessage,
    category: Option<Category>,
}

struct BitcoindLogLine {
    datetimestamp: String,
    process: String,
    message_container: BitcoindLogMessageContainer,
}

struct StructuredLogLine {
    date: String,
    timestamp: String,
    verbosity_level: String,
    component: String,
    event_template: String,
}

fn main() {
    let file_contents = fs::read_to_string("../../../Desktop/bitcoind-log.txt").expect("failed to open file");
    // println!("{}", file_contents);
    let lines: Vec<&str> = file_contents.lines().collect();

    for line in lines {
        // let first_item = bitcoind_log_parser::get_first_item_in_log_line(line);

        //println!("{}", first_item);
        let log_line = parse_line(line);
            println!("---------------");
            println!("datetimestamp: {}", log_line.datetimestamp);
            println!("process: {}", log_line.process);
            match log_line.message_container.kind {
                BitcoindLogMessage::NewOutboundPeerConnected { version, blocks, peer, peeraddr, connection_type, raw } => {
                    println!("NEW PEER!");
                    println!("version: {}", version);
                    println!("blocks: {}", blocks);
                    println!("peer: {}", peer);
                    println!("peeraddr: {}", peeraddr);
                    println!("connection_type: {}", connection_type);
                    println!("raw: {}", raw);
                }
                BitcoindLogMessage::Unknown { raw } => {
                    println!("UNKNOWN");
                    println!("raw: {}", raw);
                }
                _ => {panic!("DONT KNOW THIS KIND")}
            }

        //println!("---------------");
    }

    fn parse_line(log_line: &str) -> BitcoindLogLine {
        let log_line_seperated_by_spaces: Vec<&str> = log_line.split(" ").collect();
        let datetime = log_line_seperated_by_spaces[0].to_string();
        let process_in_brackets = log_line_seperated_by_spaces[1];
        let process: String = process_in_brackets[1..(process_in_brackets.len()-1)].to_string();
        let log_header: LogHeader = LogHeader { 
            datetimestamp: datetime,
            verbosity_level: None,
            process,
        };
        let log_message: LogMessage = log_line_seperated_by_spaces[2..].join(" ").trim().to_string();
        let log_line: LogLine = LogLine { header: log_header, message: log_message, raw: log_line.to_string() };

        let log_message_seperated_by_spaces: Vec<&str> = log_line.message.split(" ").collect();
        let first_item = log_message_seperated_by_spaces[0].to_string();
        let does_first_item_end_in_colon = first_item.chars().last().expect("No first item") == ':';
        let first_item_without_colon: String = first_item[0..(first_item.len()-1)].to_string();

        fn is_new_outbound_peer_log_line(message: &LogMessage) -> bool {
            return message.starts_with("New outbound peer connected")
        }
        
        fn parse_new_outbound_peer_message(message: &LogMessage) -> BitcoindLogMessage {
            if !is_new_outbound_peer_log_line(message) {
                panic!("BAD!");
            }

            let message_seperated_by_spaces: Vec<&str> = message.split(" ").collect();
            let version = message_seperated_by_spaces[5].strip_suffix(',').unwrap().to_string().parse().expect("fail: 1");
            let message_seperated_by_spaces_after_version = message_seperated_by_spaces[6..].to_vec();
            let mut blocks: u64 = 0;
            let mut peer: u64 = 0;
            let mut peeraddr: String = String::from("");
            let mut connection_type: String = String::from("");
            for part in message_seperated_by_spaces_after_version {
                if part.starts_with("blocks=") {
                    let key_and_value: Vec<&str> = part.split("=").collect();
                    blocks = key_and_value[1].strip_suffix(',').unwrap().to_string().parse().unwrap();
                } else if part.starts_with("peer=") {
                    let key_and_value: Vec<&str> = part.split("=").collect();
                    peer = key_and_value[1].strip_suffix(',').unwrap().to_string().parse().unwrap();
                } else if part.starts_with("peeraddr=") {
                    let key_and_value: Vec<&str> = part.split("=").collect();
                    peeraddr =key_and_value[1].to_string();
                } else if part.starts_with("(") {
                    connection_type = part.to_string().strip_prefix('(').unwrap().to_string().strip_suffix(')').unwrap().to_string();
                }
            }
            return BitcoindLogMessage::NewOutboundPeerConnected {
                version,
                blocks,
                peer,
                peeraddr,
                connection_type,
                raw: message.clone(),
            }
        }

        let bitcoind_log_message = if is_new_outbound_peer_log_line(&log_line.message) {
            parse_new_outbound_peer_message(&log_line.message)
        } else {
            BitcoindLogMessage::Unknown { raw: log_line.message }
        };

        let bitcoind_log_line = BitcoindLogLine {
            datetimestamp: log_line.header.datetimestamp,
            process: log_line.header.process,
            message_container: BitcoindLogMessageContainer {
                kind: bitcoind_log_message,
                category: get_category_type(&first_item_without_colon),
            }
        };
        return bitcoind_log_line
    }
    // let first_line = lines.get(0).unwrap().clone();
    // let rest_of_lines = &lines[0..];

    // println!("first line: {}", lines[0]);
    // println!("rest of lines: {}", rest_of_lines[2..].join(" "));

}
