// Helpful source: https://towardsdatascience.com/understand-your-comp&uter-system-using-logs-98139d0b5de1
extern crate logwatcher;
use bitcoind_log_parser;
use bitcoind_log_parser::{LogLine, LogMessage};
use logwatcher::LogWatcher;
use logwatcher::LogWatcherAction;

fn main() {
    // Including the file this way because fs::read_to_string (below) was throwing a confusing
    // error.

    /*
    let file_content = include_str!("bitcoind-log.txt");
    // let file_content: String = fs::read_to_string(content).expect("failed to open file");

    let lines: Vec<&str> = file_content.lines().collect();
    for line in lines {
        let log_line: LogLine = bitcoind_log_parser::parse(line).unwrap();
        match &log_line.message {
            LogMessage::NewOutboundPeerConnected(_) => {
                println!("{:#?}", &log_line);
            }
            LogMessage::TransactionAddedToMempool(_) => {
                println!("{:#?}", &log_line);
            }
            LogMessage::Unknown { raw: _raw } => {
                println!("{:#?}", &log_line);
            }
            _ => {
                println!("{}", line)
            }
        }
    }
    */

    type Callback = Box<dyn Fn(LogLine) + 'static>;

    struct BitcoindWatcher {
        transaction_added_to_mempool_callback: Option<Callback>,
        new_proof_of_work_valid_block: Option<Callback>,
        new_outbound_peer_connected: Option<Callback>,
    }

    impl BitcoindWatcher {
        fn run(&self) {
            let transaction_added_to_mempool_callback_ref =
                self.transaction_added_to_mempool_callback.as_ref();
            let new_proof_of_work_valid_block_ref = self.new_proof_of_work_valid_block.as_ref();
            let new_outbound_peer_connected_ref = self.new_outbound_peer_connected.as_ref();

            const FILE_TO_WATCH: &str = "/Users/joe/Library/Application Support/Bitcoin/debug.log";
            let mut log_watcher = LogWatcher::register(FILE_TO_WATCH.to_string()).unwrap();
            log_watcher.watch(&mut move |line: String| {
                let log_line: LogLine = bitcoind_log_parser::parse(&line).unwrap();
                match &log_line.message {
                    LogMessage::NewOutboundPeerConnected(_) => {
                        match new_outbound_peer_connected_ref {
                            Some(ref callback) => callback(log_line),
                            None => println!("no callback"),
                        }
                    }
                    LogMessage::TransactionAddedToMempool(tatmp) => {
                        match transaction_added_to_mempool_callback_ref {
                            Some(ref callback) => callback(log_line),
                            None => (),
                        }
                        //println!("{:#?}", &log_line);
                        //println!("{:#?}", tatmp.txid);
                    }
                    LogMessage::NewPoWValidBlock(npowvbm) => {
                        match transaction_added_to_mempool_callback_ref {
                            Some(ref callback) => callback(log_line),
                            None => (),
                        }
                        //println!("{:#?}", &log_line);
                    }
                    LogMessage::Unknown { raw: _raw } => {
                        //println!("{:#?}", &log_line);
                    }
                    _ => {
                        //println!("{}", line)
                    }
                }
                LogWatcherAction::None
            });
        }
        fn new() -> Self {
            BitcoindWatcher {
                transaction_added_to_mempool_callback: None,
                new_proof_of_work_valid_block: None,
                new_outbound_peer_connected: None,
            }
        }
        fn on_transaction_added_to_mempool(mut self, callback: Callback) -> Self {
            self.transaction_added_to_mempool_callback = Some(callback);
            self
        }
        fn on_new_proof_of_work_valid_block(mut self, callback: Callback) -> Self {
            self.new_proof_of_work_valid_block = Some(callback);
            self
        }
        fn on_new_outbound_peer_connected(mut self, callback: Callback) -> Self {
            self.new_outbound_peer_connected = Some(callback);
            self
        }
    }

    let on_transaction_added_to_mempool: Callback = Box::new(|log_line| {
        println!(".....on_transaction_added_to_mempool.....");
        println!("{:?}", log_line)
    });
    let on_new_proof_of_work_valid_block: Callback = Box::new(|log_line| {
        println!(".....on_new_proof_of_work_valid_block.....");
        println!("{:?}", log_line)
    });
    let on_new_outbound_peer_connected: Callback = Box::new(|log_line| {
        println!(".....on_new_outbound_peer_connected.....");
        println!("{:?}", log_line)
    });

    BitcoindWatcher::new()
        //.on_transaction_added_to_mempool(on_transaction_added_to_mempool)
        //.on_new_proof_of_work_valid_block(on_new_proof_of_work_valid_block)
        .on_new_outbound_peer_connected(on_new_outbound_peer_connected)
        .run();
}
