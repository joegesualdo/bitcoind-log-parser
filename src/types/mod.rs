// TODO: What's the file structure/naming convention in rust?
mod new_outbound_peer_connected_message;
pub use new_outbound_peer_connected_message::NewOutboundPeerConnectedMessage;

mod log_line;
pub use log_line::LogLine;

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
}

pub struct BitcoindLogMessageContainer {
    pub message: BitcoindLogMessage,
    pub category: Option<Category>,
}

pub struct BitcoindLogLine {
    pub datetimestamp: String,
    pub process: String,
    pub message_container: BitcoindLogMessageContainer,
}
