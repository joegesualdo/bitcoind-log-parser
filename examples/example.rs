// Helpful source: https://towardsdatascience.com/understand-your-comp&uter-system-using-logs-98139d0b5de1
use bitcoind_log_parser;
use bitcoind_log_parser::types::*;
use std::fs;

fn main() {
    let file_contents: String =
        fs::read_to_string("bitcoind-log.txt").expect("failed to open file");
    println!("here2");
    let lines: Vec<&str> = file_contents.lines().collect();

    for line in lines {
        let log_line: LogLine = bitcoind_log_parser::parse(line).unwrap();
        match &log_line.message {
            LogMessage::NewOutboundPeerConnected(_) => {
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
}
