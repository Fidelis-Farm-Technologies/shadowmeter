/*
 *  Copyright 2024 Fidelis Farm & Technologies, LLC.
 *  All Rights Reserved.
 *  See license information in LICENSE.
 */

extern crate c_string;
extern crate libc;

use c_string::CStrBuf;
use std::ffi::CStr;
use clap::Parser;
use std::fs;

// C functions wrappering libfixbuf operations
extern { 
    fn yaf2csv(input_file: *const i8, output_file: *const i8, archive_dir: *const i8); 
    fn yaf2json(input_file: *const i8, output_file: *const i8, archive_dir: *const i8); 
}

fn safe_yaf2csv(input_file: &CStr, output_dir: &CStr, archive_dir: &CStr) {
    unsafe { yaf2csv(input_file.as_ptr(), output_dir.as_ptr(), archive_dir.as_ptr()) };
}

fn safe_yaf2json(input_file: &CStr, output_dir: &CStr, archive_dir: &CStr) {
    unsafe { yaf2json(input_file.as_ptr(), output_dir.as_ptr(), archive_dir.as_ptr()) };
}


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: Option<String>,

    #[arg(long)]
    output: Option<String>,

    #[arg(long)]
    archive: Option<String>,

}

fn process_yaf_files(
    input_dir: &String, 
    output_dir: &String,
    archive_dir: &String) -> Result<(), std::io::Error> {

    let output = match CStrBuf::from_str(output_dir) {
        Ok(s) => s,
        Err(e) => panic!("{}", e)
    };

    let archive = match CStrBuf::from_str(archive_dir) {
        Ok(s) => s,
        Err(e) => panic!("{}", e)
    };

    if !input_dir.is_empty() {
        for entry in fs::read_dir(input_dir)? {
            let file = entry?;
            let filename = String::from(file.path().to_string_lossy());
            if filename.ends_with(".yaf") {
                println!("{:?}", file.path());
                let yaf_file = match CStrBuf::from_str(&filename) {
                    Ok(s) => s,
                    Err(e) => panic!("{}", e)
                };              
                safe_yaf2csv(&yaf_file, &output, &archive);        
            }            
        }
    } else {
        // process input from STDIN
        let stdin = match CStrBuf::from_str("stdin") {
            Ok(s) => s,
            Err(e) => panic!("{}", e)
        };
    
        let stdout = match CStrBuf::from_str("stdout") {
            Ok(s) => s,
            Err(e) => panic!("{}", e)
        };
        safe_yaf2csv(&stdin, &stdout, &archive);
    }

    Ok(())
}

fn main() {
    println!("shadowmeter_collector");

    let args = Args::parse();

    let input_dir = args.input.clone().unwrap_or("".to_string());
    let output_dir = args.output.clone().unwrap_or("".to_string());
    let archive_dir = args.archive.clone().unwrap_or("".to_string());

    let _ = process_yaf_files(
                &input_dir,
                &output_dir,  
                &archive_dir
    );
}
