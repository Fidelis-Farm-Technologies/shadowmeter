extern crate serde_derive;

use chrono::NaiveDateTime;
use glob::glob;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::SyncSender;

use serde_json::Value;
use std::{fs, thread, time};

pub struct YafFiles {
    sensor_id: String,
    directory_glob: String,
    processed_dir: String,
    output: SyncSender<Record>,
}

use crate::flow::Record;

impl YafFiles {
    pub fn new(
        sensor_id: &String,
        directory_glob: &String,
        processed_dir: &String,
        output: SyncSender<Record>,
    ) -> Self {
        Self {
            sensor_id: sensor_id.clone(),
            directory_glob: directory_glob.clone(),
            processed_dir: processed_dir.clone(),
            output: output,
        }
    }
    pub fn process_loop(&mut self) {
        let sleep_interval = time::Duration::from_millis(1000);
        println!("process_loop: running");
        loop {
            let mut count = 0;
            for entry in glob(&self.directory_glob).expect("Failed to read glob pattern") {
                if let Ok(path) = entry {
                    if path.is_file() {
                        let src_dir = path.parent().unwrap();
                        let src_file = path.file_name().unwrap();
                        let src = format!(
                            "{}/{}",
                            src_dir.to_str().unwrap(),
                            src_file.to_str().unwrap()
                        );
                        let dst = format!("{}/{}", &self.processed_dir, src_file.to_str().unwrap());

                        match self.process_file(&src) {
                            Ok(_success) => count = count + 1,
                            Err(error) => println!("Failed to process: {} -- {}", src, error),
                        }

                        if self.processed_dir.is_empty() {
                            fs::remove_file(src).unwrap();
                        } else {
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

    fn process_file(&mut self, file_name: &String) -> Result<(), Error> {
        println!("process_file: {}", file_name);

        let file = File::open(file_name)?;
        for line in BufReader::new(file).lines() {
            let line = line.expect("Error: reading json record");
            let v: Value = serde_json::from_str(&line).expect("Error: deserializing json record");
            let flow = &v["flows"];

            // parse start time
            let start_time_ms = NaiveDateTime::parse_from_str(
                flow["flowStartMilliseconds"].as_str().unwrap(),
                "%Y-%m-%d %H:%M:%S%.f",
            )
            .unwrap()
            .and_utc()
            .timestamp_millis()
                * 1000;

            // parse end time
            let last_time_ms = NaiveDateTime::parse_from_str(
                flow["flowEndMilliseconds"].as_str().unwrap(),
                "%Y-%m-%d %H:%M:%S%.f",
            )
            .unwrap()
            .and_utc()
            .timestamp_millis()
                * 1000;

            // source addresss
            let mut saddr = flow["sourceIPv4Address"].as_str().unwrap();
            if saddr.is_empty() {
                saddr = flow["sourceIPv6Address"].as_str().unwrap();
            }

            // destination addresss
            let mut daddr = flow["destinationIPv4Address"].as_str().unwrap();
            if daddr.is_empty() {
                daddr = flow["destinationIPv6Address"].as_str().unwrap();
            }

            // application label
            let l7_protocol = flow["ndpiL7Protocol"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let l7_sub_protocol = flow["ndpiL7SubProtocol"]
                .as_str()
                .unwrap_or("")
                .to_string();

            let mut applabel = "".to_string();

            if l7_protocol.is_empty() {
                if l7_sub_protocol != "Unknown" {
                    applabel = l7_sub_protocol.to_lowercase();
                }
            } else if l7_protocol == "Unknown" {
                if l7_sub_protocol != "Unknown" {
                    applabel = l7_sub_protocol.to_lowercase();
                }
            } else {
                applabel = format!("{l7_protocol}.{l7_sub_protocol}").to_lowercase();
            }

            let record = Record {
                sid: self.sensor_id.clone(),
                stime: start_time_ms,
                ltime: last_time_ms,
                proto: flow["protocolIdentifier"].as_i64().unwrap(),
                saddr: saddr.to_string(),
                sport: flow["sourceTransportPort"].as_i64().unwrap(),
                daddr: daddr.to_string(),
                dport: flow["destinationTransportPort"].as_i64().unwrap(),
                sasn: 0,
                sasnorg: "".to_string(),
                scountry: "".to_string(),
                dasn: 0,
                dasnorg: "".to_string(),
                dcountry: "".to_string(),
                sutcp: flow["unionTCPFlags"].as_str().unwrap_or("").to_string(),
                dutcp: flow["reverseUnionTCPFlags"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                sitcp: flow["initialTCPFlags"].as_str().unwrap_or("").to_string(),
                ditcp: flow["reverseInitialTCPFlags"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                spd: flow["firstEightNonEmptyPacketDirections"]
                    .as_str()
                    .unwrap_or("")
                    .to_string(),
                vlan: flow["vlanId"].as_i64().unwrap(),
                sdata: flow["dataByteCount"].as_i64().unwrap(),
                ddata: flow["reverseDataByteCount"].as_i64().unwrap(),
                sbytes: flow["octetTotalCount"].as_i64().unwrap(),
                dbytes: flow["reverseOctetTotalCount"].as_i64().unwrap(),
                spkts: flow["packetTotalCount"].as_i64().unwrap(),
                dpkts: flow["reversePacketTotalCount"].as_i64().unwrap(),
                sentropy: flow["payloadEntropy"].as_i64().unwrap(),
                dentropy: flow["reversePayloadEntropy"].as_i64().unwrap(),
                siat: flow["averageInterarrivalTime"].as_i64().unwrap(),
                diat: flow["reverseAverageInterarrivalTime"].as_i64().unwrap(),
                reason: flow["flowEndReason"].as_str().unwrap_or("").to_string(),
                applabel: applabel.to_string(),
                model: "".to_string(),
                score: 0.0
            };
            self.output.send(record).expect("error sending record");
        }

        Ok(())
    }
}
