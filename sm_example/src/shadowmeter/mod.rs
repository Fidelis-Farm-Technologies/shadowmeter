extern crate glob;

use std::io;
use std::io::BufRead;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread::{self, JoinHandle};

mod sink;
mod source;

pub static FORMAT_VERSION: &'static str = "100";
pub static FORMAT_VERSION_EXT: &'static str = "102";
pub static DELIMITER: &'static str = "|";

pub const CHANNEL_LENGTH: usize = 32;

pub const VERSION: usize = 0;
pub const OBSERVATION: usize = 1;
pub const STIME: usize = 2;
pub const ETIME: usize = 3;
pub const DUR: usize = 4;
pub const RTT: usize = 5;
pub const PROTO: usize = 6;
pub const SADDR: usize = 7;
pub const DADDR: usize = 8;
pub const SPORT: usize = 9;

pub const DPORT: usize = 10;
pub const IFLAG: usize = 11;
pub const RIFLAG: usize = 12;
pub const UFLAG: usize = 13;
pub const RUFLAG: usize = 14;
pub const TCPSEQ: usize = 15;
pub const RTCPSEQ: usize = 16;
pub const VLAN: usize = 17;
pub const RVLAN: usize = 18;
pub const SPKTS: usize = 19;

pub const DPKTS: usize = 20;
pub const SBYTES: usize = 21;
pub const DBYTES: usize = 22;
pub const SENTROPY: usize = 23;
pub const DENTROPY: usize = 24;
pub const SDATA: usize = 25;
pub const DDATA: usize = 26;
pub const SIAT: usize = 27;
pub const DIAT: usize = 28;
pub const SSTDEV: usize = 29;

pub const DSTDEV: usize = 30;
pub const STCPURG: usize = 31;
pub const DTCPURG: usize = 32;
pub const SSMALLPKTCNT: usize = 33;
pub const DSMALLPKTCNT: usize = 34;
pub const SNONEMPTYPKTCNT: usize = 35;
pub const DNONEMPTYPKTCNT: usize = 36;
pub const SFIRSTNONEMPTYSIZE: usize = 37;
pub const DFIRSTNONEMPTYSIZE: usize = 38;
pub const SSTDEVPAYLOAD: usize = 39;

pub const DSTDEVPAYLOAD: usize = 40;
pub const SMAXPKTSIZE: usize = 41;
pub const DMAXPKTSIZE: usize = 42;
pub const SPD: usize = 43;
pub const APPID: usize = 44;
pub const REASON: usize = 45;
pub const SMAC: usize = 46;
pub const DMAC: usize = 47;

pub const SOUI: usize = 48;
pub const DOUI: usize = 49;

pub const SCN: usize = 50;
pub const DCN: usize = 51;

pub const SASN: usize = 52;
pub const DASN: usize = 53;

pub const SASNORG: usize = 54;
pub const DASNORG: usize = 55;



pub const MAX_FIELDS: usize = 56;

pub trait Processor {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn run(&self);

    fn process_stream(&self, input_channel: Receiver<String>, output_channel: SyncSender<String>);

    fn process_pipeline3(
        &self,
        input_spec: String,
        output_spec: String,
        archive_spec: String,
        filename_spec: String, 
        interval: u64,
    ) {
        let (source_channel, input_channel) = mpsc::sync_channel::<String>(CHANNEL_LENGTH);
        let (output_channel, sink_channel) = mpsc::sync_channel::<String>(CHANNEL_LENGTH);

        thread::spawn(move || {
            if !output_spec.is_empty() {
                if output_spec.starts_with("tcp://") {
                    sink::socket(output_spec[6..].to_string(), sink_channel);
                } else if output_spec.ends_with("/") {
                    sink::directory(output_spec, filename_spec,interval, sink_channel);
                } else {
                    sink::file(output_spec, sink_channel);
                }
            } else {
                sink::stdout(sink_channel);
            }
        });

        thread::spawn(move || {
            if !input_spec.is_empty() {
                if input_spec.starts_with("tcp://") {
                    source::socket(input_spec[6..].to_string(), source_channel);
                } else if input_spec.ends_with("/") {
                    source::directory(input_spec, archive_spec, source_channel);
                } else {
                    source::file(input_spec, source_channel);
                }
            } else {
                source::stdin(source_channel);
            }
        });

        self.process_stream(input_channel, output_channel);
    }

    fn process_pipeline2(
        &self,
        input_spec: String,
        output_spec: String,
        archive_spec: String,
        filename_spec: String,
        interval: u64,
    ) {
        let (source_channel, input_channel) = mpsc::sync_channel::<String>(CHANNEL_LENGTH);
        let (output_channel, sink_channel) = mpsc::sync_channel::<String>(CHANNEL_LENGTH);

        thread::spawn(move || {
            if !output_spec.is_empty() {
                if output_spec.starts_with("tcp://") {
                    sink::socket(output_spec[6..].to_string(), sink_channel);
                } else if output_spec.ends_with("/") {
                    sink::directory(output_spec, filename_spec, interval, sink_channel);
                } else {
                    sink::file(output_spec, sink_channel);
                }
            } else {
                sink::stdout(sink_channel);
            }
        });


        thread::spawn(move || {
            if !input_spec.is_empty() {
                if input_spec.starts_with("tcp://") {
                    source::socket(input_spec[6..].to_string(), source_channel);
                } else if input_spec.ends_with("/") {
                    source::directory(input_spec, archive_spec, source_channel);
                } else {
                    source::file(input_spec, source_channel);
                }
            } else {
                source::stdin(source_channel);
            }
        });

        self.process_stream(input_channel, output_channel);
    }
}
