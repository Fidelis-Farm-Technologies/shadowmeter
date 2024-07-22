/*
 *  Copyright 2024 Fidelis Farm & Technologies, LLC.
 *  All Rights Reserved.
 *  See license information in LICENSE.
 */

extern crate c_string;
extern crate exitcode;
extern crate libc;

use clap::Parser;

use crate::shadowmeter::collect::collect;
use crate::shadowmeter::export::export;
use crate::shadowmeter::feature::feature;
use crate::shadowmeter::inference::inference;
use crate::shadowmeter::forward::forward;

mod shadowmeter;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    command: String,

    #[arg(long)]
    observation_domain: Option<String>,

    #[arg(long)]
    input: Option<String>,

    #[arg(long)]
    output: Option<String>,

    #[arg(long)]
    archive: Option<String>,

    #[arg(long)]
    poll: Option<u64>,

    #[arg(long)]
    asn: Option<String>,

    #[arg(long)]
    country: Option<String>,

    #[arg(long)]
    format: Option<String>,

    #[arg(long)]
    table: Option<String>,    
}

fn parse_command() {
    let args = Args::parse();

    let observation = args.observation_domain.clone().unwrap_or("yaf".to_string());
    let input_spec = args.input.clone().unwrap_or("".to_string());
    let output_spec = args.output.clone().unwrap_or("".to_string());
    let archive_spec = args.archive.clone().unwrap_or("".to_string());
    let format = args.format.clone().unwrap_or("json".to_string());
    let table = args.table.clone().unwrap_or("flow".to_string());

    let poll_ms = args.poll.clone().unwrap_or(0);

    match args.command.as_str() {
        "collect" => {
            let asn = args.asn.clone().unwrap_or("".to_string());
            let country = args.country.unwrap_or("".to_string());
            if !observation.chars().all(char::is_alphanumeric) {
                println!(
                    "error:  --observation-domain <string> cannot have any special characters"
                );
            } else {
                let _ = collect(
                    &observation,
                    &input_spec,
                    &output_spec,
                    &archive_spec,
                    poll_ms,
                    &asn,
                    &country,
                );
            }
        }
        "export" => {
            let _ = export(&input_spec, &output_spec, &archive_spec, &format);
        }
        "feature" => {
            let _ = feature(&input_spec, &output_spec, &archive_spec, &format);
        }
        "inference" => {
            let _ = inference(&input_spec, &output_spec, &archive_spec, &format);
        }
        "forward" => {
            let _ = forward(&input_spec, &output_spec, &archive_spec);
        }        
        _ => {
            println!("error: invalid --command <option>");
            std::process::exit(exitcode::CONFIG)
        }
    }
}
fn main() {
    parse_command();
}
