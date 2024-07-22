/*
 *  Copyright 2024 Fidelis Farm & Technologies, LLC.
 *  All Rights Reserved.
 *  See license information in LICENSE.
 */

extern crate c_string;
extern crate libc;

use c_string::CStrBuf;
use clap::Parser;
use std::ffi::CStr;
use std::fs;
use std::{thread, time};

// C functions wrappering libfixbuf operations
extern "C" {
    fn to_csv_file(
        observation: *const i8,
        input_file: *const i8,
        output_file: *const i8,
        archive_dir: *const i8,
        mac_file: *const i8,
        asn_file: *const i8,
        country_file: *const i8,
    );
}

fn safe_yaf2csv(
    observation: &CStr,
    input_file: &CStr,
    output_dir: &CStr,
    archive_dir: &CStr,
    mac_file: &CStr,
    asn_file: &CStr,
    country_file: &CStr,
) {
    unsafe {
        to_csv_file(
            observation.as_ptr(),
            input_file.as_ptr(),
            output_dir.as_ptr(),
            archive_dir.as_ptr(),
            mac_file.as_ptr(),
            asn_file.as_ptr(),
            country_file.as_ptr(),
        )
    };
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
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
    mac: Option<String>,

    #[arg(long)]
    asn: Option<String>,

    #[arg(long)]
    country: Option<String>,
}

fn process_yaf_files(
    observation_domain: &String,
    input_dir: &String,
    output_dir: &String,
    archive_dir: &String,
    poll_ms: u64,
    mac: &String,
    asn: &String,
    country: &String,
) -> Result<(), std::io::Error> {
    let mut output = match CStrBuf::from_str(output_dir) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    if output.is_empty() {
        output = match CStrBuf::from_str(".") {
            Ok(s) => s,
            Err(e) => panic!("{}", e),
        };
    }

    let archive = match CStrBuf::from_str(archive_dir) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let observation = match CStrBuf::from_str(observation_domain) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let mac = match CStrBuf::from_str(mac) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let asn = match CStrBuf::from_str(asn) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let country = match CStrBuf::from_str(country) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    if !input_dir.is_empty() {
        loop {
            for entry in fs::read_dir(input_dir)? {
                let file = entry?;
                let filename = String::from(file.path().to_string_lossy());
                if filename.ends_with(".yaf") {
                    let yaf_file = match CStrBuf::from_str(&filename) {
                        Ok(s) => s,
                        Err(e) => panic!("{}", e),
                    };
                    safe_yaf2csv(
                        &observation,
                        &yaf_file,
                        &output,
                        &archive,
                        &mac,
                        &asn,
                        &country,
                    );
                }
            }
            if poll_ms == 0 {
                break;
            }
            thread::sleep(time::Duration::from_millis(poll_ms));
        }
    } else {
        // process input from STDIN
        let stdin = match CStrBuf::from_str("stdin") {
            Ok(s) => s,
            Err(e) => panic!("{}", e),
        };
        safe_yaf2csv(&observation, &stdin, &output, &archive, &mac, &asn, &country);
    }

    Ok(())
}

fn main() {
    println!("shadowmeter_collector");

    let args = Args::parse();

    let observation = args.observation_domain.clone().unwrap_or("yaf".to_string());
    let input_dir = args.input.clone().unwrap_or("".to_string());
    let output_dir = args.output.clone().unwrap_or("".to_string());
    let archive_dir = args.archive.clone().unwrap_or("".to_string());
    let poll_ms = args.poll.clone().unwrap_or(0);
    let mac = args.mac.clone().unwrap_or("".to_string());
    let asn = args.asn.clone().unwrap_or("".to_string());
    let country = args.country.unwrap_or("".to_string());

    if !observation.chars().all(char::is_alphanumeric) {
        println!("error:  --observation-domain <string> cannot have any special characters");
    } else {
        let _ = process_yaf_files(
            &observation,
            &input_dir,
            &output_dir,
            &archive_dir,
            poll_ms,
            &mac,
            &asn,
            &country,
        );
    }
}
