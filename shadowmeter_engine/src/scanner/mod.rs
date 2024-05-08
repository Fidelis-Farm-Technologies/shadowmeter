extern crate serde_derive;

use chrono::NaiveDateTime;
use glob::glob;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::SyncSender;

use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::HashMap;
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

            let mut silk_app_label = "".to_string();
            let silk_app_id = flow["silkAppLabel"].as_i64().unwrap();
            if silk_app_id > 0 {
                if let Some(value) = SILK_LABEL_LOOKUP.get(&silk_app_id) {
                    silk_app_label = value.clone();
                }
            }

            let record = Record {
                sid: self.sensor_id.clone(),
                stime: start_time_ms,
                ltime: last_time_ms,
                proto: flow["protocolIdentifier"].as_i64().unwrap(),
                saddr: flow["sourceIPv4Address"].as_str().unwrap().to_string(),
                sport: flow["sourceTransportPort"].as_i64().unwrap(),
                daddr: flow["destinationIPv4Address"].as_str().unwrap().to_string(),
                dport: flow["destinationTransportPort"].as_i64().unwrap(),
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
                applabel: silk_app_label,
                model: "".to_string(),
                score: 0.0
            };
            self.output.send(record).expect("error sending record");
        }

        Ok(())
    }
}

lazy_static! {
    static ref SILK_LABEL_LOOKUP: HashMap<i64, String> = {
        let mut m = HashMap::new();
        m.insert(0, "?".to_string());
        m.insert(80, "http".to_string());
        m.insert(22, "ssh".to_string());
        m.insert(25, "smtp".to_string());
        m.insert(53, "dns".to_string());
        m.insert(137, "netbios".to_string());
        m.insert(443, "tls".to_string());
        m.insert(22, "ssh".to_string());
        m.insert(51443, "quic".to_string());
        m.insert(427, "slp".to_string());
        m.insert(139, "smb/netbios".to_string());
        m.insert(143, "imap".to_string());
        m.insert(194, "irc".to_string());
        m.insert(22, "ssh".to_string());
        m.insert(554, "rtsp".to_string());
        m.insert(5060, "sip".to_string());
        m.insert(873, "rsync".to_string());
        m.insert(3389, "rdp".to_string());
        m.insert(500, "ike".to_string());
        m.insert(1723, "pptp".to_string());
        m.insert(119, "nntp".to_string());
        m.insert(69, "tftp".to_string());
        m.insert(3544, "toredo".to_string());
        m.insert(3306, "mysql".to_string());
        m.insert(110, "pop".to_string());
        m.insert(161, "smtp".to_string());
        m.insert(1883, "mqtt".to_string());
        m.insert(5190, "aim".to_string());
        m.insert(6346, "gnutella".to_string());
        m.insert(5050, "yahoo msg".to_string());
        m.insert(1080, "socks".to_string());
        m.insert(179, "bgp".to_string());
        m.insert(67, "dhcp".to_string());
        m.insert(5900, "vnc/rfb".to_string());
        m.insert(5004, "rtp".to_string());
        m.insert(5005, "rtcp".to_string());
        m.insert(5222, "jabber".to_string());
        m.insert(1863, "msnp".to_string());
        m.insert(2223, "msoffice update".to_string());
        m.insert(2427, "mgcp".to_string());
        m.insert(2944, "megaco".to_string());
        m.insert(902, "vmware console".to_string());
        m.insert(6881, "bittorrent".to_string());
        m.insert(389, "ldap".to_string());
        m.insert(20000, "dnp3".to_string());
        m.insert(502, "modbus".to_string());
        m.insert(44818, "ethernet/ip".to_string());
        m.insert(138, "netbios".to_string());
        m.insert(9997, "gh0st rat".to_string());
        m.insert(6553, "pioson ivy".to_string());
        m.insert(646, "ldp".to_string());
        m.insert(65533, "palevo".to_string());
        m.insert(123, "ntp".to_string());
        m
    };
}
