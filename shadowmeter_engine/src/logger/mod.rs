extern crate serde_derive;

use questdb::{
    ingress::{Buffer, Sender, TimestampMicros, TimestampNanos},
    Result,
};

use std::sync::mpsc::{Receiver};

use crate::flow::Record;
pub struct QuestDB {
    input: Receiver<Record>,
    db_sender: Sender,
}

impl QuestDB {
    pub fn new(db_url: &str, input: Receiver<Record>) -> Self {
        let db_sender = Sender::from_conf(format!("tcp::addr={db_url};"));
        Self {
            input: input,
            db_sender: db_sender.expect("Error: failed to connecto to questdb"),
        }
    }

    pub fn process_loop(&mut self) -> Result<()> {
        loop {
            let record = self.input.recv().unwrap();
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
        }
    }
}


