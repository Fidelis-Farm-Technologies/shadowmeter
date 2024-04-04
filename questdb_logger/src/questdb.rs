extern crate serde_derive;

use chrono::NaiveDateTime;
use questdb::{
    ingress::{Buffer, Sender, TimestampMicros, TimestampNanos},
    Result,
};
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};

use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Appender {
    sensor_id: String,
    db_sender: Sender,
}

impl Appender {
    pub fn new(sensor_id: &str, db_url: &str) -> Appender {
        let db_sender = Sender::from_conf(format!("tcp::addr={db_url};"));
        Appender {
            sensor_id: sensor_id.to_string().clone(),
            db_sender: db_sender.expect("Error: failed to connecto to questdb"),
        }
    }

    pub fn process_json_file(&mut self, file_name: &String) -> Result<()> {
        
        println!("processing: {}", file_name);
        let mut buffer = Buffer::new();
        let file = File::open(file_name).expect("Error: opening json file");
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
            let end_time_ms = NaiveDateTime::parse_from_str(
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
            let _ = buffer
                .table("flow")?
                .symbol("sensorId",self.sensor_id.clone())?
                .column_ts("flowStartMilliseconds", TimestampMicros::new(start_time_ms))?
                .column_ts("flowEndMilliseconds", TimestampMicros::new(end_time_ms))?
                .column_i64(
                    "protocolIdentifier",
                    flow["protocolIdentifier"].as_i64().unwrap(),
                )?
                .column_str(
                    "sourceIPv4Address",
                    flow["sourceIPv4Address"].as_str().unwrap(),
                )?
                .column_i64(
                    "sourceTransportPort",
                    flow["sourceTransportPort"].as_i64().unwrap(),
                )?
                .column_str(
                    "destinationIPv4Address",
                    flow["destinationIPv4Address"].as_str().unwrap(),
                )?
                .column_i64(
                    "destinationTransportPort",
                    flow["destinationTransportPort"].as_i64().unwrap(),
                )?
                .column_str(
                    "unionTCPFlags",
                    flow["unionTCPFlags"].as_str().unwrap_or(""),
                )?
                .column_str(
                    "reverseUnionTCPFlags",
                    flow["reverseUnionTCPFlags"].as_str().unwrap_or(""),
                )?
                .column_str(
                    "initialTCPFlags",
                    flow["initialTCPFlags"].as_str().unwrap_or(""),
                )?
                .column_str(
                    "reverseInitialTCPFlags",
                    flow["reverseInitialTCPFlags"].as_str().unwrap_or(""),
                )?
                .column_str(
                    "firstEightNonEmptyPacketDirections",
                    flow["firstEightNonEmptyPacketDirections"]
                        .as_str()
                        .unwrap_or(""),
                )?
                .column_i64("vlanId", flow["vlanId"].as_i64().unwrap())?
                .column_i64("dataByteCount", flow["dataByteCount"].as_i64().unwrap())?
                .column_i64(
                    "reverseDataByteCount",
                    flow["reverseDataByteCount"].as_i64().unwrap(),
                )?
                .column_i64("octetTotalCount", flow["octetTotalCount"].as_i64().unwrap())?
                .column_i64(
                    "reverseOctetTotalCount",
                    flow["reverseOctetTotalCount"].as_i64().unwrap(),
                )?
                .column_i64(
                    "packetTotalCount",
                    flow["packetTotalCount"].as_i64().unwrap(),
                )?
                .column_i64(
                    "reversePacketTotalCount",
                    flow["reversePacketTotalCount"].as_i64().unwrap(),
                )?
                .column_i64("payloadEntropy", flow["payloadEntropy"].as_i64().unwrap())?
                .column_i64(
                    "reversePayloadEntropy",
                    flow["reversePayloadEntropy"].as_i64().unwrap(),
                )?
                .column_str(
                    "flowEndReason",
                    flow["flowEndReason"].as_str().unwrap_or(""),
                )?
                .column_str("AppLabel", silk_app_label)?
                .at(TimestampNanos::now())
                .unwrap();

            let _ = self.db_sender.flush(&mut buffer).unwrap();
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
