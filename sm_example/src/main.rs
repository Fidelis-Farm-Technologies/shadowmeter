mod shadowmeter;

use crate::shadowmeter::Processor;
use clap::Parser;
use geoip2::{Country, Reader, ASN};
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::{fs, net::IpAddr, str::FromStr};
use mac_oui::Oui;

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: Option<String>,

    #[arg(long)]
    output: Option<String>,

    #[arg(long)]
    archive: Option<String>,

    #[arg(long)]
    file_prefix: Option<String>,

    #[arg(long)]
    interval: Option<u64>,

    #[arg(long)]
    oui: Option<String>,

    #[arg(long)]
    asn: Option<String>,

    #[arg(long)]
    country: Option<String>,
}

const PRIVATE_IP_RANGE: [&'static str; 18] = [
    "10.", "192.168.", "172.16.", "172.17.", "172.18.", "172.19.", "172.20.", "172.21.", "172.22.",
    "172.23.", "172.24.", "172.25.", "172.26.", "172.27.", "172.28.", "172.29.", "172.30.",
    "172.31.",
];

struct SmExample {
    input: String,
    output: String,
    archive: String,
    file_prefix: String,
    interval: u64,
    oui: String,
    asn: String,
    country: String,
    name: &'static str,
}

impl SmExample {
    fn is_private_address(&self, ipaddr: &str) -> bool {
        for i in 0..PRIVATE_IP_RANGE.len() {
            if ipaddr.starts_with(PRIVATE_IP_RANGE[i]) {
                return true;
            }
        }
        return false;
    }
    fn append_asn(&self, asn_db: &Reader<ASN>, saddr: &str, daddr: &str, extension: &mut String) {
        let mut sasn: i64 = 0;
        let mut sasnorg: String = String::new();
        let mut dasn: i64 = 0;
        let mut dasnorg: String = String::new();

        if !self.is_private_address(saddr) {
            if let Ok(ip) = IpAddr::from_str(saddr) {
                if let Ok(result) = asn_db.lookup(ip) {
                    sasn = i64::from(result.autonomous_system_number.unwrap());
                    /*
                    sasnorg = result
                        .autonomous_system_organization
                        .unwrap_or("")
                        .to_lowercase()
                        .to_string();
                    */
                }
            }
        }

        if !self.is_private_address(daddr) {
            if let Ok(ip) = IpAddr::from_str(daddr) {
                if let Ok(result) = asn_db.lookup(ip) {
                    dasn = i64::from(result.autonomous_system_number.unwrap());
                    /*
                    dasnorg = result
                        .autonomous_system_organization
                        .unwrap_or("")
                        .to_lowercase()
                        .to_string();
                    */
                }
            }
        }
        extension.push_str(shadowmeter::DELIMITER);
        extension.push_str(&sasn.to_string());
        extension.push_str(shadowmeter::DELIMITER);
        extension.push_str(&dasn.to_string());
        //extension.push_str(",");
        //extension.push_str(sasnorg.to_string());
        //extension.push_str(",");
        //extension.push_str(dasnorg.to_string());
    }

    fn append_country(
        &self,
        country_db: &Reader<Country>,
        saddr: &str,
        daddr: &str,
        extension: &mut String,
    ) {
        let mut scountry: String = String::new();
        let mut dcountry: String = String::new();

        if !self.is_private_address(saddr) {
            if let Ok(ip) = IpAddr::from_str(saddr) {
                if let Ok(result) = country_db.lookup(ip) {
                    if let Some(country) = result.country {
                        scountry = country.iso_code.unwrap_or("").to_lowercase().to_string();
                    }
                }
            }
        }

        if !self.is_private_address(daddr) {
            if let Ok(ip) = IpAddr::from_str(daddr) {
                if let Ok(result) = country_db.lookup(ip) {
                    if let Some(country) = result.country {
                        dcountry = country.iso_code.unwrap_or("").to_lowercase().to_string();
                    }
                }
            }
        }

        extension.push_str(shadowmeter::DELIMITER);
        extension.push_str(&scountry.to_string());
        extension.push_str(shadowmeter::DELIMITER);
        extension.push_str(&dcountry.to_string());
    }

    fn append_oui(&self, oui_db: &Oui, smac: &str, dmac: &str, extension: &mut String) {
        
        let mut soui: String = String::new();
        let mut doui: String = String::new();

        let res = oui_db.lookup_by_mac(&smac);
        match res {
            Ok(r) => {
                if let Some(rec) = r {
                   soui = rec.company_name.clone();
                }
            }
            Err(e) => println!("Error: {}", e),
        }
        let res = oui_db.lookup_by_mac(&dmac);
        match res {
            Ok(r) => {
                if let Some(rec) = r {
                   doui = rec.company_name.clone();
                }
            }
            Err(e) => println!("Error: {}", e),
        }
        extension.push_str(shadowmeter::DELIMITER);
        extension.push_str(&smac.to_string());
        extension.push_str(shadowmeter::DELIMITER);
        extension.push_str(&dmac.to_string());
    }
}

impl shadowmeter::Processor for SmExample {
    fn new(name: &'static str) -> SmExample {
        let args = Args::parse();
        SmExample {
            input: args.input.clone().unwrap_or("".to_string()),
            output: args.output.clone().unwrap_or("".to_string()),
            archive: args.archive.clone().unwrap_or("".to_string()),
            file_prefix: args.file_prefix.clone().unwrap_or("sm.".to_string()),
            interval: args.interval.clone().unwrap_or(10),
            oui: args.oui.clone().unwrap_or("".to_string()),
            asn: args.asn.clone().unwrap_or("".to_string()),
            country: args.country.clone().unwrap_or("".to_string()),
            name: name,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn run(&self) {
        self.process_pipeline3(
            self.input.clone(),
            self.output.clone(),
            self.archive.clone(),
            self.file_prefix.clone(),
            self.interval,
        );
    }

    fn process_stream(&self, input_channel: Receiver<String>, output_channel: SyncSender<String>) {
        let mut counter: u32 = 0;

        let asn_vector = match std::fs::read(self.asn.to_string()) {
            Ok(vec) => vec,
            Err(e) => panic!("Unable to open ASN file: {e:?}"),
        };
        let asn_db = match Reader::<ASN>::from_bytes(&asn_vector) {
            Ok(db) => db,
            Err(e) => panic!("Unable to load ASN data: {e:?}"),
        };

        let country_vector = match std::fs::read(self.country.to_string()) {
            Ok(vec) => vec,
            Err(e) => panic!("Unable to open ASN file: {e:?}"),
        };
        let country_db: Reader<Country> = match Reader::<Country>::from_bytes(&country_vector) {
            Ok(db) => db,
            Err(e) => panic!("Unable to load Country data: {e:?}"),
        };

        let oui_db = match Oui::default() {
            Ok(db) => db,
            Err(e) => panic!("Unable to load OUI data: {e:?}"),
        };

        loop {
            match input_channel.recv() {
                Ok(mut line) => {
                    let field = line.trim().split(',').collect::<Vec<&str>>();
                    if field.is_empty() {
                        continue;
                    } else if field[shadowmeter::VERSION].starts_with("ver")
                        || field[shadowmeter::VERSION].starts_with(shadowmeter::FORMAT_VERSION)
                    {
                        let mut extension = String::new();

                        if oui_db.get_total_records() > 0 {
                            self.append_oui(
                                &oui_db,
                                field[shadowmeter::SMAC],
                                field[shadowmeter::DMAC],
                                &mut extension,
                            );
                        }

                        if !country_vector.is_empty() {
                            self.append_country(
                                &country_db,
                                field[shadowmeter::SADDR],
                                field[shadowmeter::SADDR],
                                &mut extension,
                            );
                        }

                        if !asn_vector.is_empty() {
                            self.append_asn(
                                &asn_db,
                                field[shadowmeter::SADDR],
                                field[shadowmeter::SADDR],
                                &mut extension,
                            );
                        }

                        line.push_str(extension.as_str());
                        //println!("{}", line);
                        output_channel.send(line).expect("error sending record");

                        counter += 1;
                    } else {
                        panic!("invalid data");
                    }
                }
                Err(_eof) => {
                    break;
                }
            };
        }
        println!("processed {} records", counter);
    }
}

fn main() {
    use std::time::Instant;

    let mut sm_example: SmExample = shadowmeter::Processor::new("sm_example");
    println!("{} running", sm_example.name());

    let mark = Instant::now();
    sm_example.run();
    println!("elapse time: {:.2?}", mark.elapsed());
}
