//
//
//
//

extern crate glob;

use clap::Parser;
use glob::glob;

use std::{fs, thread, time};

mod questdb;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    questdb: String,

    #[arg(short, long)]
    sensor_id: String,
}


fn main() {
    let args = Args::parse();
    let mut dest_dir = String::new();
    if !args.output.is_none() {
        dest_dir = args.output.clone().unwrap();
    }

    let mut db = questdb::Appender::new(&args.sensor_id, &args.questdb);
    let sleep_interval = time::Duration::from_millis(1000);
    println!("questdb_logger: running");
    loop {
        let mut count = 0;
        for entry in glob(&args.input).expect("Failed to read glob pattern") {
            if let Ok(path) = entry {
                if path.is_file() {
                    let src_dir = path.parent().unwrap();
                    let src_file = path.file_name().unwrap();
                    let src = format!(
                        "{}/{}",
                        src_dir.to_str().unwrap(),
                        src_file.to_str().unwrap()
                    );
                    let dst = format!("{}/{}", &dest_dir, src_file.to_str().unwrap());

                    match db.process_json_file(&src) {
                        Ok(_success) => count = count + 1,
                        Err(error) => println!("Failed to process: {} -- {}", src, error),
                    }

                    if args.output.is_none() {
                        //println!("removed: {} ", src);
                        fs::remove_file(src).unwrap();
                    } else {
                        //println!("moved: src: {}, dst: {} ", src, dst);
                        fs::rename(src, dst).unwrap();
                    }
                    count = count + 1;
                }
            }
        }
        if count == 0 {
            thread::sleep(sleep_interval);
        }
    }
}
