use glob::glob;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use questdb::{
    ingress::{Buffer, Sender, TimestampMicros, TimestampNanos},
    Result,
};


use duckdb::Connection;


fn insert_questdb_records(db_in: &String, db_out: &mut Sender, table: &String) {
    let mut buffer = Buffer::new();
    /*
        let _ = buffer
            .table("flow")?
            .symbol("sid", record.sid)?
            .symbol("applabel", record.applabel)?
            .symbol("spd", record.spd)?
            .symbol("reason", record.reason)?
            .symbol("sasnorg", record.sasnorg)?
            .symbol("scountry", record.scountry)?
            .symbol("dasnorg", record.dasnorg)?
            .symbol("dcountry", record.dcountry)?
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
    */
    db_out.flush(&mut buffer)
        .unwrap();
}

pub fn export_questdb(db_in: &String, db_out: &String, archive_spec: &String, table: &String) {
    println!("db in: {}", db_in);
    println!("db out: {}", db_out);
    println!("archive directory: {}", archive_spec);

    match Sender::from_conf(format!("tcp::addr={db_out};")) {
        Ok(mut db_out) => insert_questdb_records(db_in, &mut db_out, table),
        Err(e) => panic!("error: openining flow file: {e:?}"),
    };
}

pub fn export_file(input_file: &String, output_file: &String, format: &String) {
    //println!("export file: {} => {}", input_file, output_file);

    let conn = match Connection::open(input_file) {
        Ok(c) => c,
        Err(e) => panic!("error: openining flow file: {e:?}"),
    };

    let sql_command;
    match format.as_str() {
        "csv" => {
            if !output_file.ends_with("csv") {
                println!("exporting {} => {}.csv", input_file, output_file);
                sql_command = format!(
                    "COPY (SELECT * FROM flow) TO '{}.csv' (HEADER, DELIMITER ',');",
                    output_file
                );
            } else {
                println!("exporting {} => {}", input_file, output_file);
                sql_command = format!(
                    "COPY (SELECT * FROM flow) TO '{}' (HEADER, DELIMITER ',');",
                    output_file
                );
            }
        }
        "parquet" => {
            println!("exporting {} => {}", input_file, output_file);
            sql_command = format!(
                "COPY (SELECT * FROM flow) TO '{}' (FORMAT 'parquet');",
                output_file
            );
        }
        "questdb" => {
            println!("exporting {} => {}", input_file, output_file);
            sql_command = format!("COPY (SELECT * FROM flow) TO '{}';", output_file);
        }        
        "json" => {
            println!("exporting {} => {}", input_file, output_file);
            sql_command = format!("COPY (SELECT * FROM flow) TO '{}';", output_file);
        }
        _ => {
            println!("exporting {} => {}", input_file, output_file);
            sql_command = format!("COPY (SELECT * FROM flow) TO '{}';", output_file);
        }
    }

    //println!("sql: {}", sql_command);
    match conn.execute_batch(&sql_command) {
        Ok(c) => c,
        Err(e) => panic!("error: openining flow file: {e:?}"),
    };
}

pub fn export(input_spec: &String, output_spec: &String, archive_spec: &String, format: &String) {
    //println!("input directory: {}", input_spec);
    //println!("output directory: {}", output_spec);
    //println!("archive directory: {}", archive_spec);
    //println!("format: {}", format);

    if PathBuf::from(input_spec.clone()).is_dir() && PathBuf::from(output_spec.clone()).is_dir() {
        let sleep_interval = Duration::from_millis(1000);
        println!("directory scanner: running");
        loop {
            let mut count = 0;
            for entry in glob(&input_spec).expect("Failed to read glob pattern") {
                if let Ok(path) = entry {
                    if path.is_file() {
                        let src_dir = path.parent().unwrap();
                        let src_file = path.file_name().unwrap();
                        let src_path = format!(
                            "{}/{}",
                            src_dir.to_str().unwrap(),
                            src_file.to_str().unwrap()
                        );

                        let dst_path =
                            format!("{}/{}.{}", output_spec, src_file.to_str().unwrap(), format);

                        if src_path.ends_with(".flow") {
                            export_file(&src_path, &dst_path, format);
                        }

                        if archive_spec.is_empty() {
                            fs::remove_file(src_path).unwrap();
                        } else {
                            let dst_path =
                                format!("{}/{}", &archive_spec, src_file.to_str().unwrap());
                            fs::rename(src_path, dst_path).unwrap();
                        }
                        count = count + 1;
                    }
                }
            }
            if count == 0 {
                thread::sleep(sleep_interval);
            }
        }
    } else if PathBuf::from(input_spec.clone()).is_file()
        && !PathBuf::from(output_spec.clone()).is_dir()
    {
        export_file(input_spec, output_spec, format);
    } else {
        panic!("export command error: invalid input spec and output spec");
    }
}
