/*
 *  Copyright 2024 Fidelis Farm & Technologies, LLC.
 *  All Rights Reserved.
 *  See license information in LICENSE.
 */

extern crate c_string;
extern crate libc;

use c_string::CStrBuf;
use std::ffi::CStr;
use std::fs;
use std::path::Path;
use std::time::Duration;
use std::{thread, time};

// C functions wrappering libfixbuf operations
extern "C" {
    fn yaf_import(
        observation: *const i8,
        input_file: *const i8,
        output_file: *const i8,
        asn_file: *const i8,
        country_file: *const i8,
    ) -> i32;
}

fn safe_yaf_import(
    observation: &CStr,
    input_file: &CStr,
    output_file: &CStr,
    asn_file: &CStr,
    country_file: &CStr,
) -> i32 {
    unsafe {
        return yaf_import(
            observation.as_ptr(),
            input_file.as_ptr(),
            output_file.as_ptr(),
            asn_file.as_ptr(),
            country_file.as_ptr(),
        );
    };
}

pub fn import(
    observation_domain: &String,
    input_spec: &String,
    output_spec: &String,
    processed_spec: &String,
    polling: bool,
    asn_spec: &String,
    country_spec: &String,
) -> Result<(), std::io::Error> {

    let input = match CStrBuf::from_str(input_spec) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let output = match CStrBuf::from_str(output_spec) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let processed = match CStrBuf::from_str(processed_spec) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let observation = match CStrBuf::from_str(observation_domain) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let country = match CStrBuf::from_str(country_spec) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let asn = match CStrBuf::from_str(asn_spec) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    println!("\tinput spec: {}", input_spec);
    println!("\toutput spec: {}", output_spec);
    println!("\tprocessed spec: {}", processed_spec);
    println!("\tasn file: {}", asn_spec);
    println!("\tcountry file: {}", country_spec);
    println!("\tpolling: {}", polling);
    
    if Path::new(input_spec).is_file() {
        let status = safe_yaf_import(&observation, &input, &output, &asn, &country);
        if status < 0 {
            eprintln!("error: processing {}", input_spec);
            std::process::exit(exitcode::DATAERR);
        }
    } else {
        let poll_interval = Duration::from_millis(1000);
        println!("import scanner: running [{}]", input_spec);
        loop {
            let mut counter = 0;
            let mut processed_path;
           
            for entry in fs::read_dir(input_spec)? {
                let file: fs::DirEntry = entry.unwrap();
                let file_name = String::from(file.file_name().to_string_lossy());
                let src_path = String::from(file.path().to_string_lossy());

                if src_path.ends_with(".yaf") {
                    let lock_path = format!("{}.lock", src_path);
                    if Path::new(lock_path.as_str()).exists() {
                        continue;
                    }

                    let yaf_file = match CStrBuf::from_str(&src_path) {
                        Ok(s) => s,
                        Err(e) => panic!("{}", e),
                    };
                    let status = safe_yaf_import(&observation, &yaf_file, &output, &asn, &country);
                    if status < 0 {
                        eprintln!(
                            "error: processing {}; moving to {}",
                            src_path, processed_spec
                        );
                        processed_path = format!("{}/{}.err", processed_spec, file_name);
                    } else {
                        println!("processed: {}", src_path);
                        processed_path = format!("{}/{}", processed_spec, file_name);
                    }
                    if !processed_spec.is_empty() {
                        match fs::rename(src_path.clone(), processed_path.clone()) {
                            Ok(c) => c,
                            Err(e) => {
                                panic!("error: moving {} -> {}: {:?}", src_path, processed_path, e)
                            }
                        };
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
    }
    Ok(())
}
