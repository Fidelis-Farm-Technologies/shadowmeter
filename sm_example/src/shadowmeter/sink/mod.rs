extern crate glob;

use chrono;
use std::fs::File;
use std::io::BufRead;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

use std::io::{self, BufWriter, Write};

pub fn socket(output_spec: String, recv_channel: Receiver<String>) {
    println!("output tcp: {}", output_spec);

    let mut stream = TcpStream::connect(output_spec).expect("tcp connection failed");
    loop {
        let line = match recv_channel.recv() {
            Ok(line) => stream
                .write_all(line.as_bytes())
                .expect("socket write failed"),
            Err(_eof) => {
                break;
            }
        };
    }
}

pub fn file(output_spec: String, recv_channel: Receiver<String>) {
    println!("output file: {}", output_spec);
    let output = File::create(output_spec).expect("creating output fil failed");
    let mut output = BufWriter::new(output);

    loop {
        let line = match recv_channel.recv() {
            Ok(line) => output.write_all(line.as_bytes()).expect("write failed"),
            Err(_eof) => {
                break;
            }
        };
    }
}

pub fn directory(
    output_spec: String,
    archive_spec: String,
    interval: u32,
    recv_channel: Receiver<String>,
) {
    let now = chrono::offset::Local::now();
    let file_name = now.format("sm%Y%m%y.%H%M%S.csv");
    let file_path = format!("{}{}", output_spec, file_name);
    println!("output directory: {}", file_path);
}

pub fn stdout(recv_channel: Receiver<String>) {
    println!("stdout:");
    loop {
        let line = match recv_channel.recv() {
            Ok(line) => print!("{}", line),
            Err(_eof) => {
                break;
            }
        };
    }
}
