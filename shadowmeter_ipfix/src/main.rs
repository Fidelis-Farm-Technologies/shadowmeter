/*
 *  Copyright 2024 Fidelis Farm & Technologies, LLC.
 *  All Rights Reserved.
 *  See license information in LICENSE.
 */

extern crate c_string;
extern crate libc;

use c_string::CStrBuf;
use std::ffi::CStr;

extern { 
    fn yaf2csv(input_file: *const i8, output_file: *const i8); 
    fn yaf2json(input_file: *const i8, output_file: *const i8); 
}


fn safe_yaf2csv(input_file: &CStr, output_file: &CStr) {
    unsafe { yaf2csv(input_file.as_ptr(), output_file.as_ptr()) };
}

fn safe_yaf2json(input_file: &CStr, output_file: &CStr) {
    unsafe { yaf2json(input_file.as_ptr(), output_file.as_ptr()) };
}


fn main() {
    println!("shadowmeter_ipfix");

    let input_file = match CStrBuf::from_str("/var/shadowmeter/input") {
        Ok(s) => s,
        Err(e) => panic!("{}", e)
    };
    let output_file = match CStrBuf::from_str("/var/shadowmeter/output") {
        Ok(s) => s,
        Err(e) => panic!("{}", e)
    };

    safe_yaf2csv(&input_file, &output_file);

    safe_yaf2json(&input_file, &output_file);

}
