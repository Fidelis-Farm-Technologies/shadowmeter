extern crate serde_derive;

use geoip2::{Reader, ASN};
use std::{fs, net::IpAddr, str::FromStr};

use questdb::{
    ingress::{Buffer, Sender, TimestampMicros, TimestampNanos},
    Result,
};

use std::sync::mpsc::Receiver;

use crate::flow::Record;

pub struct Database {
    input: Receiver<Record>,
    db_sender: Sender,
    geolite_file: String,
}

impl Database {
    pub fn new(db_url: &String, geolite_file: &String, input: Receiver<Record>) -> Self {
        let db_sender = Sender::from_conf(format!("tcp::addr={db_url};"));
        Self {
            input: input,
            db_sender: db_sender.expect("error: failed to connecto to questdb"),
            geolite_file: geolite_file.clone(),
        }
    }

    pub fn process_with_asn(&mut self, data: Vec<u8>) {
        println!("ASN tagging: enabled");
        let geolite_asn = Reader::<ASN>::from_bytes(&data).unwrap();

        loop {
            let mut record = self.input.recv().unwrap();

            // lookup saddr ASN
            let mut ipaddr = IpAddr::from_str(record.saddr.as_str());
            match ipaddr {
                Ok(ip) => {
                    let mut query = geolite_asn.lookup(ip);
                    match query {
                        Ok(result) => {
                            record.sasn = i64::from(result.autonomous_system_number.unwrap());
                            record.sasnorg =
                                result.autonomous_system_organization.unwrap().to_string();
                        }
                        Err(_) => (),
                    }
                }
                Err(_) => (),
            }

            // lookup daddr ASN
            ipaddr = IpAddr::from_str(record.daddr.as_str());
            match ipaddr {
                Ok(ip) => {
                    let mut query = geolite_asn.lookup(ip);
                    match query {
                        Ok(result) => {
                            record.dasn = i64::from(result.autonomous_system_number.unwrap());
                            record.dasnorg =
                                result.autonomous_system_organization.unwrap().to_string();
                        }
                        Err(_) => (),
                    }
                }
                Err(_) => (),
            }

            let _ = self.insert_record(record);
        }
    }

    pub fn process_without_asn(&mut self) {
        println!("ASN tagging: disabled");
        loop {
            let record = self.input.recv().unwrap();
            let _ = self.insert_record(record);
        }
    }

    pub fn process_loop(&mut self) -> Result<()> {
        let buffer = std::fs::read(self.geolite_file.to_string());
        match buffer {
            Ok(data) => { 
                if data.len() > 0 {
                    Ok(self.process_with_asn(data))
                } else {
                    Ok(self.process_without_asn())
                }},
            Err(e) => Ok(self.process_without_asn()),
        }
    }

    pub fn insert_record(&mut self, record: Record) -> Result<()> {
        let mut buffer = Buffer::new();
        let _ = buffer
            .table("flow")?
            .symbol("sid", record.sid)?
            .column_ts("stime", TimestampMicros::new(record.stime))?
            .column_ts("ltime", TimestampMicros::new(record.ltime))?
            .column_i64("proto", record.proto)?
            .column_str("saddr", record.saddr)?
            .column_i64("sport", record.sport)?
            .column_str("daddr", record.daddr)?
            .column_i64("dport", record.dport)?
            .column_i64("sasn", record.sasn)?
            .column_str("sasnorg", record.sasnorg)?
            .column_i64("dasn", record.dasn)?
            .column_str("dasnorg", record.dasnorg)?
            .column_str("sutcp", record.sutcp)?
            .column_str("dutcp", record.dutcp)?
            .column_str("sitcp", record.sitcp)?
            .column_str("ditcp", record.ditcp)?
            .column_str("spd", record.spd)?
            .column_i64("vlan", record.vlan)?
            .column_i64("sdata", record.sdata)?
            .column_i64("ddata", record.ddata)?
            .column_i64("sbytes", record.sbytes)?
            .column_i64("dbytes", record.dbytes)?
            .column_i64("spkts", record.spkts)?
            .column_i64("dpkts", record.dpkts)?
            .column_i64("sentropy", record.sentropy)?
            .column_i64("dentropy", record.dentropy)?
            .column_i64("siat", record.siat)?
            .column_i64("diat", record.diat)?
            .column_str("reason", record.reason)?
            .column_str("applabel", record.applabel)?
            .at(TimestampNanos::now())
            .unwrap();

        let _ = self.db_sender.flush(&mut buffer).unwrap();

        Ok(())
    }
}
