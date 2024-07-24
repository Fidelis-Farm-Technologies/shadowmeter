/*
 *  Copyright 2024 Fidelis Farm & Technologies, LLC.
 *  All Rights Reserved.
 *  See license information in LICENSE.
 */

use questdb::{
    ingress::{Buffer, Sender, TimestampMicros, TimestampNanos},
    Result,
};

use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use duckdb::Connection;

#[derive(Debug)]
struct FlowRecord {
    id: i64,
    observ: String,
    stime: i64,
    etime: i64,
    dur: f64,
    rtt: f64,
    proto: i64,
    addr: String,
    raddr: String,
    port: i64,
    rport: i64,
    iflag: String,
    riflag: String,
    uflag: String,
    ruflag: String,
    tcpseq: String,
    rtcpseq: String,
    vlan: i64,
    rvlan: i64,
    pkts: i64,
    rpkts: i64,
    bytes: i64,
    rbytes: i64,
    entropy: i64,
    rentropy: i64,
    data: i64,
    rdata: i64,
    iat: i64,
    riat: i64,
    stdev: i64,
    rstdev: i64,
    tcpurg: i64,
    rtcpurg: i64,
    smallpktcnt: i64,
    rsmallpktcnt: i64,
    nonemptypktcnt: i64,
    rnonemptypktcnt: i64,
    firstnonemptysize: i64,
    rfirstnonemptysize: i64,
    maxpktsize: i64,
    rmaxpktsize: i64,
    stdevpayload: i64,
    rstdevpayload: i64,
    spd: String,
    appid: String,
    reason: String,
    mac: String,
    rmac: String,
    country: String,
    rcountry: String,
    asn: i64,
    rasn: i64,
    asnorg: String,
    rasnorg: String,
}

fn insert_questdb_records(db_in: &Connection, db_out: &mut questdb::ingress::Sender) {
    let mut buffer = Buffer::new();
    let mut stmt = db_in.prepare("SELECT * FROM flow;").unwrap();
    let record_iter = stmt
        .query_map([], |row| {
            Ok(FlowRecord {
                id: row.get(0).unwrap(),
                observ: row.get(1).unwrap(),
                stime: row.get(2).unwrap(),
                etime: row.get(3).unwrap(),
                dur: row.get(4).unwrap(),
                rtt: row.get(5).unwrap(),
                proto: row.get(6).unwrap(),
                addr: row.get(7).unwrap(),
                raddr: row.get(8).unwrap(),
                port: row.get(9).unwrap(),
                rport: row.get(10).unwrap(),
                iflag: row.get(11).unwrap(),
                riflag: row.get(12).unwrap(),
                uflag: row.get(13).unwrap(),
                ruflag: row.get(14).unwrap(),
                tcpseq: row.get(15).unwrap(),
                rtcpseq: row.get(16).unwrap(),
                vlan: row.get(17).unwrap(),
                rvlan: row.get(18).unwrap(),
                pkts: row.get(19).unwrap(),
                rpkts: row.get(20).unwrap(),
                bytes: row.get(21).unwrap(),
                rbytes: row.get(22).unwrap(),
                entropy: row.get(23).unwrap(),
                rentropy: row.get(24).unwrap(),
                data: row.get(25).unwrap(),
                rdata: row.get(26).unwrap(),
                iat: row.get(27).unwrap(),
                riat: row.get(28).unwrap(),
                stdev: row.get(29).unwrap(),
                rstdev: row.get(30).unwrap(),
                tcpurg: row.get(31).unwrap(),
                rtcpurg: row.get(32).unwrap(),
                smallpktcnt: row.get(33).unwrap(),
                rsmallpktcnt: row.get(34).unwrap(),
                nonemptypktcnt: row.get(35).unwrap(),
                rnonemptypktcnt: row.get(36).unwrap(),
                firstnonemptysize: row.get(37).unwrap(),
                rfirstnonemptysize: row.get(38).unwrap(),

                maxpktsize: row.get(39).unwrap(),
                rmaxpktsize: row.get(40).unwrap(),
                stdevpayload: row.get(41).unwrap(),
                rstdevpayload: row.get(42).unwrap(),
                spd: row.get(43).unwrap(),
                appid: row.get(44).unwrap(),
                reason: row.get(45).unwrap(),

                mac: row.get(46).unwrap(),
                rmac: row.get(47).unwrap(),
                country: row.get(48).unwrap(),
                rcountry: row.get(49).unwrap(),
                asn: row.get(50).unwrap(),
                rasn: row.get(51).unwrap(),
                asnorg: row.get(52).unwrap(),
                rasnorg: row.get(53).unwrap(),
            })
        })
        .unwrap();

    for r in record_iter {
        let record = r.unwrap();
        //println!("flow record: {:?}", record);

        let _ = buffer
            .table("flow").unwrap()
            .symbol("observ", record.observ).unwrap()
            .symbol("applabel", record.appid).unwrap()
            .symbol("spd", record.spd).unwrap()
            .symbol("reason", record.reason).unwrap()
            .symbol("asnorg", record.asnorg).unwrap()
            .symbol("rasnorg", record.rasnorg).unwrap()
            .symbol("country", record.country).unwrap()
            .symbol("rcountry", record.rcountry).unwrap()
            .column_ts("stime", TimestampMicros::new(record.stime)).unwrap()
            .column_ts("etime", TimestampMicros::new(record.etime)).unwrap()
            .column_i64("vlan", record.vlan).unwrap()
            .column_i64("rvlan", record.rvlan).unwrap()            
            .column_i64("proto", record.proto).unwrap()
            .column_str("addr", record.addr).unwrap()
            .column_str("raddr", record.raddr).unwrap()            
            .column_i64("port", record.port).unwrap()
            .column_i64("rport", record.rport).unwrap()
            .column_i64("asn", record.asn).unwrap()
            .column_i64("rasn", record.rasn).unwrap()            
            .column_str("iflag", record.iflag).unwrap()
            .column_str("riflag", record.riflag).unwrap()
            .column_str("uflag", record.uflag).unwrap()
            .column_str("ruflag", record.ruflag).unwrap()
            .column_str("tcpseq", record.tcpseq).unwrap()
            .column_str("rtcpseq", record.rtcpseq).unwrap()
            .column_i64("vlan", record.vlan).unwrap()
            .column_i64("rvlan", record.rvlan).unwrap()
            .column_i64("pkts", record.pkts).unwrap()
            .column_i64("rpkts", record.rpkts).unwrap()
            .column_i64("bytes", record.bytes).unwrap()
            .column_i64("rbytes", record.rbytes).unwrap()
            .column_i64("entropy", record.entropy).unwrap()
            .column_i64("rentropy", record.rentropy).unwrap()
            .column_i64("data", record.data).unwrap()
            .column_i64("rdate", record.rdata).unwrap()
            .column_i64("iat", record.iat).unwrap()
            .column_i64("riat", record.riat).unwrap()
            .column_i64("stdev", record.stdev).unwrap()
            .column_i64("rstdev", record.rstdev).unwrap()
            .column_i64("tcpurg", record.tcpurg).unwrap()
            .column_i64("rtcpurg", record.rtcpurg).unwrap()
            .column_i64("smallpktcnt", record.smallpktcnt).unwrap()
            .column_i64("rsmallpktcnt", record.rsmallpktcnt).unwrap()
            .column_i64("nonemptypktcnt", record.nonemptypktcnt).unwrap()
            .column_i64("rnonemptypktcnt", record.rnonemptypktcnt).unwrap()
            .column_i64("firstnonemptysize", record.firstnonemptysize).unwrap()
            .column_i64("rfirstnonemptysize", record.rfirstnonemptysize).unwrap()
            .column_i64("maxpktsize", record.maxpktsize).unwrap()
            .column_i64("rmaxpktsize", record.rmaxpktsize).unwrap()
            .column_i64("stdevpayload", record.stdevpayload).unwrap()
            .column_i64("rstdevpayload", record.rstdevpayload).unwrap()
            .column_str("mac", record.mac).unwrap()
            .column_str("rmac", record.rmac).unwrap()
            .column_i64("stdevpayload", record.stdevpayload).unwrap()
            .column_i64("rstdevpayload", record.rstdevpayload).unwrap()            
            .at(TimestampNanos::now())
            .unwrap();

        db_out.flush(&mut buffer).unwrap();
    }
}

pub fn export_questdb(db_in: &Connection, db_out: &String) -> bool {
    match Sender::from_conf(format!("tcp::addr={db_out};")) {
        Ok(mut db_out) => insert_questdb_records(db_in, &mut db_out),
        Err(e) => {
            eprintln!("error: openining flow file: {e:?}");
            return false;
        },
    };
    true
}

pub fn export_file(input_spec: &String, output_spec: &String, format: &String) -> bool {
    let conn = match Connection::open(input_spec) {
        Ok(s) => s,
        Err(e) => panic!("error:  export_questdb - {}", e),
    };

    let sql_command;
    match format.as_str() {
        "csv" => {
            if !output_spec.ends_with("csv") {
                println!("exporting {} => {}.csv", input_spec, output_spec);
                sql_command = format!(
                    "COPY (SELECT * FROM flow) TO '{}.csv' (HEADER, DELIMITER ',');",
                    output_spec
                );
            } else {
                println!("exporting {} => {}", input_spec, output_spec);
                sql_command = format!(
                    "COPY (SELECT * FROM flow) TO '{}' (HEADER, DELIMITER ',');",
                    output_spec
                );
            }
        }
        "parquet" => {
            println!("exporting {} => {}", input_spec, output_spec);
            conn.execute_batch("INSTALL parquet; LOAD parquet;")
                .unwrap();
            sql_command = format!(
                "COPY (SELECT * FROM flow) TO '{}' (FORMAT 'parquet');",
                output_spec
            );
        }
        "questdb" => {
            println!("exporting {} => {}", input_spec, output_spec);
            let status = export_questdb(&conn, output_spec);
            //let mut stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
            //sql_command = format!("COPY (SELECT * FROM flow) TO '{}';", output_spec);
            return true;
        }
        "json" => {
            println!("exporting {} => {}", input_spec, output_spec);
            sql_command = format!("COPY (SELECT * FROM flow) TO '{}';", output_spec);
        }
        _ => {
            println!("exporting {} => {}", input_spec, output_spec);
            sql_command = format!("COPY (SELECT * FROM flow) TO '{}';", output_spec);
        }
    }

    match conn.execute_batch(&sql_command) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: exporting file {} -- {:?}", input_spec, e);
            return false;
        }
    };
    true
}

pub fn export(
    input_spec: &String,
    output_spec: &String,
    processed_spec: &String,
    polling: bool,
    format: &String,
) {
    if PathBuf::from(input_spec.clone()).is_dir() {
        let poll_interval = Duration::from_millis(1000);
        println!("directory scanner: running");
        loop {
            let mut counter = 0;
            let directory = match fs::read_dir(input_spec) {
                Ok(d) => d,
                Err(e) => panic!("error: reading directory {} -- {:?}", input_spec, e),
            };

            for entry in directory {
                let file = entry.unwrap();
                let file_name = String::from(file.file_name().to_string_lossy());
                let src_path = String::from(file.path().to_string_lossy());

                if src_path.ends_with(".flow") {
                    let mut dst_spec;
                    if format == "questdb" {
                        dst_spec = output_spec.clone();
                    } else {
                        dst_spec = format!("{}/{}.{}", output_spec, file_name, format);
                    }

                    if export_file(&src_path, &dst_spec, format) {
                        if !processed_spec.is_empty() {
                            let processed_path =
                                format!("{}/{}", &processed_spec, file_name.to_string());

                            match fs::rename(src_path.clone(), processed_path.clone()) {
                                Ok(c) => c,
                                Err(e) => panic!(
                                    "error: moving {} -> {}: {:?}",
                                    src_path, processed_path, e
                                ),
                            };
                        }
                    } else {
                        eprintln!("error: exporting {} => {}", src_path, dst_spec);
                        std::process::exit(exitcode::PROTOCOL);
                    }
                }
                counter += 1;
            }
            if !polling {
                break;
            }
            if counter == 0 {
                thread::sleep(poll_interval);
            }
        }
    } else {
        export_file(input_spec, output_spec, format);
    }
}
