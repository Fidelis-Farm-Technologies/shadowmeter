/*
 *  Copyright 2024 Fidelis Farm & Technologies, LLC.
 *  All Rights Reserved.
 *  See license information in LICENSE.
 */

use reqwest::blocking::Client;

use std::fs;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::{fmt::format, fs::File};

//
// TODO: add support for S3 using duckdb library
//

fn post_file(input_spec: &String, url_spec: &String) -> bool {
    let path = Path::new(input_spec);
    let file_name = path.file_stem().unwrap();
    let file_ext = path.extension().unwrap();

    let mut file = match File::open(input_spec) {
        Ok(f) => {
            if f.metadata().unwrap().len() > 1073741824 {
                eprint!("error: file {} exceeds 1GB", input_spec);
                return false;
            }
            f
        }
        Err(_) => {
            eprint!("error: opening {}", input_spec);
            return false;
        }
    };

    let mut buffer = vec![];
    file.read_to_end(&mut buffer).unwrap();
   //println!("file size: {}", buffer.len());

    let client = reqwest::blocking::Client::new();
    let res = match client.post(url_spec).body(buffer).send() {
        Ok(r) => r,
        Err(e) => {
            eprintln!(
                "error: posting {} => {} - {}",
                input_spec, url_spec, e); 
            std::process::exit(exitcode::PROTOCOL);
        }
    };
    let response_status = res.status().to_string();
    if !response_status.starts_with("200") {
        eprintln!(
            "error: posting {} => {}",
            input_spec, url_spec);            
        std::process::exit(exitcode::PROTOCOL);
    }
    println!("posted: [{}] {}", response_status, input_spec);
    true
}

pub fn post(input_spec: &String, url_spec: &String, processed_spec: &String, polling: bool) -> Result<(), std::io::Error> {
    println!("input directory: {}", input_spec);
    println!("url directory: {}", url_spec);
    println!("processed directory: {}", processed_spec);
    let sleep_interval = Duration::from_millis(1000);

    if PathBuf::from(input_spec.clone()).is_dir() {
        let poll_interval = Duration::from_millis(1000);
        println!("directory scanner: running");
        loop {
            let mut counter = 0;
            for entry in fs::read_dir(input_spec)? {
                let file = entry.unwrap();
                let file_name = String::from(file.file_name().to_string_lossy());
                let src_path = String::from(file.path().to_string_lossy());

                if src_path.ends_with(".flow") {
                    if post_file(&src_path, &url_spec) {
                        if !processed_spec.is_empty() {
                            let processed_path = 
                                format!("{}/{}", &processed_spec, file_name.to_string());

                            match fs::rename(src_path.clone(), processed_path.clone()) {
                                Ok(c) => c,
                                Err(e) => panic!(
                                    "error: moving {} -> {}: {:?}",
                                    src_path, processed_path, e
                                ),
                            };
                        }                        
                    }
                    else {
                        eprintln!(
                            "error: posting {} => {}",
                            src_path, url_spec
                        );
                        std::process::exit(exitcode::PROTOCOL);
                    }
                }
                counter += 1;
            } 
            if !polling {
                break;
            }
            if counter == 0 {
                thread::sleep(poll_interval);
            }
        }
    } else {
        post_file(input_spec, url_spec);
    }
    Ok(())
}
