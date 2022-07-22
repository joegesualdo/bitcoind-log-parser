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

    const FILE_TO_WATCH: &str = "/Users/joe/Library/Application Support/Bitcoin/debug.log";
    let mut log_watcher = LogWatcher::register(FILE_TO_WATCH.to_string()).unwrap();

    log_watcher.watch(&mut move |line: String| {
        let log_line: LogLine = bitcoind_log_parser::parse(&line).unwrap();
        match &log_line.message {
            LogMessage::NewOutboundPeerConnected(_) => {
                //println!("{:#?}", &log_line);
            }
            LogMessage::TransactionAddedToMempool(tatmp) => {
                //println!("{:#?}", &log_line);
                println!("{:#?}", tatmp.txid);
            }
            LogMessage::Unknown { raw: _raw } => {
                // println!("{:#?}", &log_line);
            }
            _ => {
                println!("{}", line)
            }
        }
        LogWatcherAction::None
    });
}
