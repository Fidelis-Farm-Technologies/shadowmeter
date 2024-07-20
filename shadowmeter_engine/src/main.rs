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
    geolite_asn: Option<String>,

    #[arg(short, long)]
    geolite_country: Option<String>,

}

fn run_with_analyzer(
    input: &String,
    database: &String,
    geolite_asn_file: &String,
    geolite_country_file: &String,
    model_file: &String,
    dest_dir: &String,
) {
    let (scanner_send, analyzer_recv) = mpsc::sync_channel::<Record>(8);
    let (analyzer_send, logger_recv) = mpsc::sync_channel::<Record>(8);

    let mut scanner = scanner::YafFiles::new(&input, &dest_dir, scanner_send);
    let mut analyzer = analyzer::Analyzer::new(analyzer_recv, analyzer_send, &model_file);
    let mut logger = logger::Database::new(&database, &geolite_asn_file, &geolite_country_file, logger_recv);

    thread::spawn(move || {
        let _ = analyzer.process_loop();
    });

    thread::spawn(move || {
        let _ = logger.process_loop();
    });

    scanner.process_loop();
}

fn run_without_analyzer(
    input: &String, 
    database: &String, 
    geolite_asn_file: &String, 
    geolite_country_file: &String, 
    dest_dir: &String
) {

    let (scanner_send, logger_recv) = mpsc::sync_channel::<Record>(8);

    let mut scanner = scanner::YafFiles::new(&input, &dest_dir, scanner_send);
    let mut logger = logger::Database::new(&database, &geolite_asn_file, &geolite_country_file, logger_recv);

    thread::spawn(move || {
        let _ = logger.process_loop();
    });

    scanner.process_loop();
}

fn main() {
    let args = Args::parse();

    let dest_dir = args.output.clone().unwrap_or("".to_string());
    let geolite_asn_file = args.geolite_asn.clone().unwrap_or("".to_string());    
    let geolite_country_file = args.geolite_country.clone().unwrap_or("".to_string());       
    let model_file = args.model.clone().unwrap_or("".to_string());

    if model_file.is_empty() {
        run_without_analyzer(

            &args.input, 
            &args.database,  
            &geolite_asn_file,  
            &geolite_country_file, 
            &dest_dir
        );
    } else {
        run_with_analyzer(
            &args.input,
            &args.database,
            &geolite_asn_file,
            &geolite_country_file, 
            &model_file,
            &dest_dir,
        );
    }
}
