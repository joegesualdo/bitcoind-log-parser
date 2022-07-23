use crate::log_line::log_message::message::Message;
use crate::log_line::log_message::parse_error::ParseError;
use crate::log_line::log_message::parse_result::ParseResult;
use crate::utils::get_key_value_from_key_value_string;

#[derive(Debug)]
pub struct TransactionAddedToMempoolMessage {
    pub txid: String,
    pub wtxid: String,
}

impl Message<TransactionAddedToMempoolMessage> for TransactionAddedToMempoolMessage {
    fn is_valid(message: &str) -> bool {
        return message.starts_with("TransactionAddedToMempool:");
    }

    fn parse(message: &str) -> ParseResult<TransactionAddedToMempoolMessage> {
        if !Self::is_valid(message) {
            return Err(ParseError);
        }
        // TODO: use a safer implementation. Getting by index could panic.
        let message_parts: Vec<&str> = message.split_whitespace().collect();
        let txid_part = message_parts[1];
        let txid_key_and_value = get_key_value_from_key_value_string(txid_part);
        let txid = txid_key_and_value[1];
        let wtxid_part = message_parts[2];
        let wtxid_key_and_value = get_key_value_from_key_value_string(wtxid_part);
        let wtxid = wtxid_key_and_value[1];

        Ok(TransactionAddedToMempoolMessage {
            txid: txid.to_string(),
            wtxid: wtxid.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_transaction_added_to_mempool_log_line_fn_works() {
        let raw_tatmp_message = "TransactionAddedToMempool: txid=b3446439365c3086600c7a0cb30b1a30add804ec59f9d12c9b95802ff7055e6a wtxid=cced41d7b159568d4dfb08e7d1ed86918df02e4a8d8751f100878911a1a5a154";
        let is_tatmp = TransactionAddedToMempoolMessage::is_valid(raw_tatmp_message);
        assert_eq!(is_tatmp, true)
    }
    #[test]
    fn parse_fnx_works() {
        let raw_tatmp_message = "TransactionAddedToMempool: txid=b3446439365c3086600c7a0cb30b1a30add804ec59f9d12c9b95802ff7055e6a wtxid=cced41d7b159568d4dfb08e7d1ed86918df02e4a8d8751f100878911a1a5a154";
        let tatmp_message = TransactionAddedToMempoolMessage::parse(raw_tatmp_message).unwrap();
        assert_eq!(
            tatmp_message.txid,
            "b3446439365c3086600c7a0cb30b1a30add804ec59f9d12c9b95802ff7055e6a"
        );
        assert_eq!(
            tatmp_message.wtxid,
            "cced41d7b159568d4dfb08e7d1ed86918df02e4a8d8751f100878911a1a5a154"
        )
    }
}
