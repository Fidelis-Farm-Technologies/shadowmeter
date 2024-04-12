//
//
//
//

extern crate glob;

use crate::flow::Record;
use clap::Parser;
use std::sync::mpsc;
use std::thread;

mod analyzer;
mod flow;
mod logger;
mod scanner;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    model: Option<String>,

    #[arg(short, long)]
    database: String,

    #[arg(short, long)]
    sensor_id: String,
}

fn main() {
    let args = Args::parse();
    
    let dest_dir = args.output.clone().unwrap_or("".to_string());
    let model_file = args.model.clone().unwrap_or("".to_string());

    let (scanner_send, analyzer_recv) = mpsc::sync_channel::<Record>(8);
    let (analyzer_send, logger_recv) = mpsc::sync_channel::<Record>(8);

    let mut scanner = scanner::YafFiles::new(&args.sensor_id, &args.input, &dest_dir, scanner_send);
    let mut analyzer = analyzer::Analyzer::new(analyzer_recv, analyzer_send, &model_file);
    let mut logger = logger::Database::new(&args.database, logger_recv);

    thread::spawn(move || {
        let _ = analyzer.process_loop();
    });

    thread::spawn(move || {
        let _ = logger.process_loop();
    });
   
    scanner.process_loop();
}
