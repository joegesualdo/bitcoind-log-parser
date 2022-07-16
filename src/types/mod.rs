// TODO: What's the file structure/naming convention in rust?
mod new_outbound_peer_connected_message;
pub use new_outbound_peer_connected_message::NewOutboundPeerConnectedMessage;

mod log_line;
pub use log_line::LogLine;

// DEBUG CATEGORIES:
//    Sources:
//      - https://bitcoin.stackexchange.com/questions/66892/what-are-the-debug-categories
//      - https://man.archlinux.org/man/community/bitcoin-daemon/bitcoind.1.en
const ADDRMAN_CATEGORY: &str = "addrman";
const BENCH_CATEGORY: &str = "bench";
const BLOCKSTORAGE_CATEGORY: &str = "blockstorage";
const CMPCTBLOCK_CATEGORY: &str = "cmpctblock";
const COINDB_CATEGORY: &str = "coindb";
const ESTIMATEFEE_CATEGORY: &str = "estimatefee";
const HTTP_CATEGORY: &str = "http";
const I2P_CATEGORY: &str = "i2p";
const IPC_CATEGORY: &str = "ipc";
const LEVELDB_CATEGORY: &str = "leveldb";
const LIBEVENT_CATEGORY: &str = "libevent";
const MEMPOOL_CATEGORY: &str = "mempool";
const MEMPOOLREJ_CATEGORY: &str = "mempoolrej";
const NET_CATEGORY: &str = "net";
const PROXY_CATEGORY: &str = "proxy";
const PRUNE_CATEGORY: &str = "prune";
const QT_CATEGORY: &str = "qt";
const RAND_CATEGORY: &str = "rand";
const REINDEX_CATEGORY: &str = "reindex";
const RPC_CATEGORY: &str = "rpc";
const SELECTCOINS_CATEGORY: &str = "selectcoins";
const TOR_CATEGORY: &str = "tor";
const UTIL_CATEGORY: &str = "util";
const VALIDATION_CATEGORY: &str = "validation";
const WALLETDB_CATEGORY: &str = "walletdb";
const ZMQ_CATEGORY: &str = "zmq";

#[derive(Debug)]
pub enum BitcoindLogMessage {
    Unknown { raw: String },
    NewOutboundPeerConnected(NewOutboundPeerConnectedMessage), // https://github.com/bitcoin/bitcoin/blob/87d012324afa285221073540781295f1b7381a15/src/net_processing.cpp#L2992
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
//TODO: Implement fmt::Display so we can go from type to the original string
//
#[derive(Debug)]
pub enum Category {
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

impl Category {
    pub fn get_category_type(category: &str) -> Option<Category> {
        match category {
            ADDRMAN_CATEGORY => Some(Category::Addrman),
            BENCH_CATEGORY => Some(Category::Bench),
            BLOCKSTORAGE_CATEGORY => Some(Category::Blockstorage),
            CMPCTBLOCK_CATEGORY => Some(Category::Cmpctblock),
            COINDB_CATEGORY => Some(Category::Coindb),
            ESTIMATEFEE_CATEGORY => Some(Category::Estimatefee),
            HTTP_CATEGORY => Some(Category::Http),
            I2P_CATEGORY => Some(Category::I2p),
            IPC_CATEGORY => Some(Category::Ipc),
            LEVELDB_CATEGORY => Some(Category::Leveldb),
            LIBEVENT_CATEGORY => Some(Category::Libevent),
            MEMPOOL_CATEGORY => Some(Category::Mempool),
            MEMPOOLREJ_CATEGORY => Some(Category::Mempoolrej),
            NET_CATEGORY => Some(Category::Net),
            PROXY_CATEGORY => Some(Category::Proxy),
            PRUNE_CATEGORY => Some(Category::Prune),
            QT_CATEGORY => Some(Category::Qt),
            RAND_CATEGORY => Some(Category::Rand),
            REINDEX_CATEGORY => Some(Category::Reindex),
            RPC_CATEGORY => Some(Category::Rpc),
            SELECTCOINS_CATEGORY => Some(Category::Selectcoins),
            TOR_CATEGORY => Some(Category::Tor),
            UTIL_CATEGORY => Some(Category::Util),
            VALIDATION_CATEGORY => Some(Category::Validation),
            WALLETDB_CATEGORY => Some(Category::Walletdb),
            ZMQ_CATEGORY => Some(Category::Zmq),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct BitcoindLogMessageContainer {
    pub message: BitcoindLogMessage,
    pub category: Option<Category>,
}

#[derive(Debug)]
pub struct BitcoindLogLine {
    pub datetimestamp: String,
    pub process: String,
    pub message_container: BitcoindLogMessageContainer,
}

impl BitcoindLogLine {
    pub fn parse(log_line_string: &str) -> BitcoindLogLine {
        let log_line = LogLine::parse_into_log_line(log_line_string);
        BitcoindLogLine::parse_log_line(log_line)
    }
    pub fn parse_log_line(log_line: LogLine) -> BitcoindLogLine {
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
                let nopcm = NewOutboundPeerConnectedMessage::parse(&log_line.message);
                match nopcm {
                    Some(n) => BitcoindLogMessage::NewOutboundPeerConnected(n),
                    None => panic!("Ut oh"),
                }
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
}
