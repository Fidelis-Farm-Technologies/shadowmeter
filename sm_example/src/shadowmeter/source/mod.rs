extern crate glob;

use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::SyncSender;
use std::thread;

mod file_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}

fn accept_connection(stream: TcpStream, send_channel: SyncSender<String>) {
    let mut reader = BufReader::new(stream);
    println!("connected");
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                //let mut field: Vec<String> = line.trim().split(',').map(str::to_string).collect();
                //let _ = self.process_line(&mut field);
                // send_channel
                //counter += 1;
                send_channel.send(line).expect("error sending record");
            }
            Err(_eof) => {
                break;
            }
        }
    }
    println!("connection closed");
}

pub fn socket(input_spec: String, send_channel: SyncSender<String>) {
    println!("input tcp: {}", input_spec);
    let listener = TcpListener::bind(input_spec).expect("failed to bind socket");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                accept_connection(stream, send_channel.clone());
            }
            Err(e) => {
                break;
            }
        }
    }
}

pub fn file(input_spec: String, send_channel: SyncSender<String>) {
    println!("input file: {}", input_spec);
    let mut reader = file_reader::BufReader::open(input_spec).unwrap();
    let mut buffer = String::new();
    //let mut counter = 0;

    while let Some(data) = reader.read_line(&mut buffer) {
        let line: String = data.unwrap().to_string();
        //let mut field: Vec<String> = line.trim().split(',').map(str::to_string).collect();
        //let _ = self.process_line(&mut field);
        // send_channel
        // counter += 1;
        send_channel.send(line).expect("error sending record");
    }
}

pub fn directory(
    input_spec: String,
    archive_spec: String,
    interval: u32,
    send_channel: SyncSender<String>,
) {
    println!("input irectory: {}", input_spec);
}

pub fn stdin(send_channel: SyncSender<String>) {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    println!("stdin:");
    loop {
        let mut line = String::new();
        match handle.read_line(&mut line) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                //let mut field: Vec<String> = line.trim().split(',').map(str::to_string).collect();
                //let _ = self.process_line(&mut field);
                // send_channel
                //counter += 1;
                send_channel.send(line).expect("error sending record");
            }
            Err(_eof) => {
                break;
            }
        }
    }
}
