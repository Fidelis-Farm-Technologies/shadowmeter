mod shadowmeter;

use crate::shadowmeter::Processor;
use clap::Parser;
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: Option<String>,

    #[arg(long)]
    output: Option<String>,

    #[arg(long)]
    archive: Option<String>,

    #[arg(long)]
    interval: Option<u32>,

    #[arg(long)]
    ou: Option<String>,

    #[arg(long)]
    geolite_asn: Option<String>,

    #[arg(long)]
    geolite_country: Option<String>,
}

struct SmExample {
    input: String,
    output: String,
    archive: String,
    interval: u32,
    ou: String,
    asn: String,
    country: String,
    name: &'static str,
}

impl SmExample {}

impl shadowmeter::Processor for SmExample {
    fn new(name: &'static str) -> SmExample {
        let args = Args::parse();
        SmExample {
            input: args.input.clone().unwrap_or("".to_string()),
            output: args.output.clone().unwrap_or("".to_string()),
            archive: args.archive.clone().unwrap_or("".to_string()),
            interval: args.interval.clone().unwrap_or(0),
            ou: args.ou.clone().unwrap_or("".to_string()),
            asn: args.geolite_asn.clone().unwrap_or("".to_string()),
            country: args.geolite_country.clone().unwrap_or("".to_string()),
            name: name,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn run(&self) {
        self.process_loop(
            self.input.clone(),
            self.output.clone(),
            self.archive.clone(),
            self.interval,
        );
    }

    fn process_stream(&self, input_channel: Receiver<String>, output_channel: SyncSender<String>) {
        let mut counter: u32 = 0;

        loop {
            //let mut line = input_channel.recv().unwrap();
            let mut line = match input_channel.recv() {
                Ok(line) => {
                    /*
                    let mut field: Vec<String> =
                        line.trim().split(',').map(str::to_string).collect();

                    if !field.is_empty() {
                        continue;
                    }
                    if field[shadowmeter::VERSION].starts_with("ver") {
                        continue;
                    }
                    if field[shadowmeter::VERSION].starts_with(shadowmeter::FORMAT_VERSION) {
                        //
                        counter += 1;
                    } else {
                        panic!("invalid data");
                    }
                    */
                    output_channel.send(line).expect("error sending record");
                }
                Err(_eof) => {
                    break;
                }
            };
        }
    }
}

fn main() {
    let mut sm_example: SmExample = shadowmeter::Processor::new("sm_example");
    println!("{} running", sm_example.name());

    sm_example.run();
}
