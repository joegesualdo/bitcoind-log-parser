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
