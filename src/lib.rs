//! # Bitcoind Log Parser
//!
//! Parse Bitcoind logs
//!

struct LogLine {
    unix_timestamp: String,
    message: String,
}

enum LogLineType {
    NewOutboundPeerConnected, // https://github.com/bitcoin/bitcoin/blob/87d012324afa285221073540781295f1b7381a15/src/net_processing.cpp#L2992
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

/// Given a log line, this will return the first item
///
/// ```
/// use bitcoind_log_parser::get_first_item_in_log_line;
///
/// let log_line = String::from(
///     "2022-07-08T17:33:16Z FlushStateToDisk: write coins cache to disk (849914 coins, 123039kB) started"
/// );
/// let first_item = get_first_item_in_log_line(&log_line);
///
/// assert!(first_item.contains("2022-07-08T17:33:16Z"));
/// ```
pub fn get_first_item_in_log_line(log_line: &str) -> &str {
    log_line.split_whitespace().next().expect("error!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first_item_in_log_line_test() {
        let log_line = String::from(
            "2021-07-08T17:33:16Z FlushStateToDisk: write coins cache to disk (849914 coins, 123039kB) started"
        );
        let first_item = get_first_item_in_log_line(&log_line);
        assert!(first_item.contains("2021-07-08T17:33:16Z"));
    }
}
