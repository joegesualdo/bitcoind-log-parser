mod message;
mod new_outbound_peer_connected_message;
mod new_pow_valid_block_message;
mod parse_error;
mod parse_result;
mod transaction_added_to_mempool_message;
use message::Message;
use new_outbound_peer_connected_message::NewOutboundPeerConnectedMessage;
use new_pow_valid_block_message::NewPoWValidBlockMessage;
use parse_error::ParseError;
use transaction_added_to_mempool_message::TransactionAddedToMempoolMessage;

#[derive(Debug)]
pub enum LogMessage {
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
    TransactionAddedToMempool(TransactionAddedToMempoolMessage), // https://github.com/bitcoin/bitcoin/blob/66e3b16b8b1033414f843058f360e22b725d89c5/src/validationinterface.cpp#L209
    NewPoWValidBlock(NewPoWValidBlockMessage), // https://github.com/bitcoin/bitcoin/blob/66e3b16b8b1033414f843058f360e22b725d89c5/src/validationinterface.cpp#L257
    // DOCS about inv: https://developer.bitcoin.org/reference/p2p_networking.html#inv
    GotInvTx, // https://github.com/bitcoin/bitcoin/blob/948f5ba6363fcc64f95fed3f04dbda3d50d61827/src/net_processing.cpp#L3237 OR https://github.com/bitcoin/bitcoin/blob/948f5ba6363fcc64f95fed3f04dbda3d50d61827/src/net_processing.cpp#L3256
    GotInvWtx,
}

impl LogMessage {
    pub fn parse(raw_log_message: String) -> Result<Self, ParseError> {
        if NewOutboundPeerConnectedMessage::is_valid(&raw_log_message) {
            // TODO: Switch this to return a Result, instead of an Option.
            let nopcm = NewOutboundPeerConnectedMessage::parse(&raw_log_message);
            match nopcm {
                Ok(n) => Ok(Self::NewOutboundPeerConnected(n)),
                Err(err) => Err(err),
            }
        } else if TransactionAddedToMempoolMessage::is_valid(&raw_log_message) {
            let tatmp = TransactionAddedToMempoolMessage::parse(&raw_log_message);
            match tatmp {
                Ok(t) => Ok(Self::TransactionAddedToMempool(t)),
                Err(err) => Err(err),
            }
        } else if NewPoWValidBlockMessage::is_valid(&raw_log_message) {
            let npowvbm = NewPoWValidBlockMessage::parse(&raw_log_message);
            match npowvbm {
                Ok(t) => Ok(Self::NewPoWValidBlock(t)),
                Err(_) => Err(ParseError),
            }
        } else {
            Ok(Self::Unknown {
                raw: raw_log_message,
            })
        }
    }
}
