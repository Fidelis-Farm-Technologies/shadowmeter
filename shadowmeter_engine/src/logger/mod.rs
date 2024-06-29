extern crate serde_derive;

use geoip2::{Reader, ASN, Country};
use std::{fs, net::IpAddr, str::FromStr};

use questdb::{
    ingress::{Buffer, Sender, TimestampMicros, TimestampNanos},
    Result,
};

use std::sync::mpsc::Receiver;

use crate::flow::Record;

const PRIVATE_IP_RANGE: [&'static str; 18] = [
    "10.", "192.168.", "172.16.", "172.17.", "172.18.", "172.19.", "172.20.", "172.21.", "172.22.",
    "172.23.", "172.24.", "172.25.", "172.26.", "172.27.", "172.28.", "172.29.", "172.30.",
    "172.31.",
];

pub struct Database {
    input: Receiver<Record>,
    db_sender: Sender,
    geolite_asn_file: String,
    geolite_country_file: String,
}

impl Database {
    pub fn new(db_url: &String, geolite_asn_file: &String, geolite_country_file: &String, input: Receiver<Record>) -> Self {
        let db_sender = Sender::from_conf(format!("tcp::addr={db_url};"));
        Self {
            input: input,
            db_sender: db_sender.expect("error: failed to connecto to questdb"),
            geolite_asn_file: geolite_asn_file.clone(),
            geolite_country_file: geolite_country_file.clone(),
        }
    }

    pub fn is_private_address(&mut self, ipaddr: &String) -> bool {
        for i in 0..PRIVATE_IP_RANGE.len() {
            if ipaddr.starts_with(PRIVATE_IP_RANGE[i]) {
                return true;
            }
        }
        return false;
    }

    pub fn process_loop(&mut self) -> Result<()> {

        let asn_data = std::fs::read(self.geolite_asn_file.to_string());
        let asn_vector = asn_data.unwrap();
        let asn_db = Reader::<ASN>::from_bytes(&asn_vector);
        if asn_db.is_ok() {
            println!("ASN tagging: enabled");
        }

        let country_data = std::fs::read(self.geolite_country_file.to_string());
        let country_vector = country_data.unwrap();
        let country_db = Reader::<Country>::from_bytes(&country_vector);
        if country_db.is_ok() {
            println!("Country tagging: enabled");
        }

        loop {
            let mut record = self.input.recv().unwrap();

            if record.saddr.is_empty() || record.daddr.is_empty() {
                // skip non IP protocols
                continue;
            }

            if !self.is_private_address(&record.saddr) {      
                if let Ok(ip) = IpAddr::from_str(record.saddr.as_str()) {  
                    // lookup saddr ASN   
                    if asn_db.is_ok() {
                        if let Ok(result) = asn_db.as_ref().unwrap().lookup(ip) {
                                record.sasn = i64::from(result.autonomous_system_number.unwrap());
                                record.sasnorg =
                                    result.autonomous_system_organization.unwrap_or("").to_lowercase().to_string();
                        }                               
                    }
                    // lookup saddr country code
                    if country_db.is_ok() {
                        if let Ok(result) = country_db.as_ref().unwrap().lookup(ip) {
                            record.scountry = result.country.unwrap().iso_code.unwrap_or("").to_string();                                  
                        }  
                    }                      
                }                
            }
            
            if !self.is_private_address(&record.daddr) {   
                if let Ok(ip) = IpAddr::from_str(record.daddr.as_str()) {                        
                    // lookup daddr ASN   
                    if asn_db.is_ok() {
                        if let Ok(result) = asn_db.as_ref().unwrap().lookup(ip) {
                                record.dasn = i64::from(result.autonomous_system_number.unwrap());
                                record.dasnorg =
                                    result.autonomous_system_organization.unwrap_or("").to_lowercase().to_string();
                        }                               
                    }
                    // lookup daddr country code               
                    if country_db.is_ok() {
                        if let Ok(result) = country_db.as_ref().unwrap().lookup(ip) {
                            record.dcountry = result.country.unwrap().iso_code.unwrap_or("").to_string();                                  
                        }  
                    }                                           
                }
                    
            }            
            
            let _ = self.insert_record(record);
        }
    }

    pub fn insert_record(&mut self, record: Record) -> Result<()> {
        let mut buffer = Buffer::new();
        let _ = buffer
            .table("flow")?
            .symbol("sid", record.sid)?
            .symbol("sasnorg", record.sasnorg)?
            .symbol("scountry", record.scountry)?  
            .symbol("dasnorg", record.dasnorg)?
            .symbol("dcountry", record.dcountry)? 
            .symbol("reason", record.reason)?    
            .symbol("applabel", record.applabel)?   
            .symbol("spd", record.spd)?                  
            .column_ts("stime", TimestampMicros::new(record.stime))?
            .column_ts("ltime", TimestampMicros::new(record.ltime))?
            .column_i64("vlan", record.vlan)?             
            .column_i64("proto", record.proto)?
            .column_str("saddr", record.saddr)?
            .column_i64("sport", record.sport)?
            .column_str("daddr", record.daddr)?
            .column_i64("dport", record.dport)?                 
            .column_str("sutcp", record.sutcp)?
            .column_str("dutcp", record.dutcp)?
            .column_str("sitcp", record.sitcp)?
            .column_str("ditcp", record.ditcp)?
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
            .column_i64("sasn", record.sasn)?
            .column_i64("dasn", record.dasn)?          
            .at(TimestampNanos::now())
            .unwrap();

        match self.db_sender.flush(&mut buffer) {
            Ok(_) => Ok(()),
            Err(e) =>  { 
                println!("DB insert error: {}", e);
                Err (e)
            },
        }
    }
}
