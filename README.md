☠️⚠️ Work In Progress ⚠️☠️

# Bitcoind Log Parser
> Parse bitcoind logs into sensible data structures

## Install
> Add package to Cargo.toml file
```rust
[dependencies]
bitcoind-log-parser = "0.1.2"
```

## Usage:
```rust
let bitcoind_log_line = "2022-07-13T15:03:16Z [msghand] New outbound peer connected: version: 70016, blocks=744841, peer=303, peeraddr=15.188.83.52:8333 (block-relay-only)";

// Parse the log line string into a LogLine datastructure
let log_line: LogLine = bitcoind_log_parser::parse(bitcoind_log_line).unwrap();

// Match the type of log line and do something based on it
match &log_line.message {
    LogMessage::NewOutboundPeerConnected(_) => {
        // do something if the log line represent a new outbound peer connected
	println!("{:#?}", &log_line);
    }
    LogMessage::TransactionAddedToMempool(_) => {
        // do something if the log line represent a transaction has been added to the mempool
	println!("{:#?}", &log_line);
    }
    LogMessage::Unknown { raw: _raw } => {
        // do something if the log line isn't recognized by bitcoind-log-parser
	println!("{:#?}", &log_line);
    }
    _ => {
	println!("{}", line)
    }
}
```

## Related
- [bitcoind-watcher](https://github.com/joegesualdo/bitcoind-watcher) - Be notified when important things happen on the Bitcoin blockchain

## License
MIT © [Joe Gesualdo]()
