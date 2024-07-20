extern crate glob;

use std::io;
use std::io::BufRead;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;

mod sink;
mod source;

pub static FORMAT_VERSION: &'static str = "100";

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
pub const SMAC: usize = 11;
pub const DMAC: usize = 12;
pub const IFLAG: usize = 13;
pub const RIFLAG: usize = 14;
pub const UFLAG: usize = 15;
pub const RUFLAG: usize = 16;
pub const TCPSEQ: usize = 17;
pub const RTCPSEQ: usize = 18;
pub const VLAN: usize = 19;
pub const RVLAN: usize = 20;
pub const SPKTS: usize = 21;
pub const DPKTS: usize = 22;
pub const SBYTES: usize = 23;
pub const DBYTES: usize = 24;
pub const SENTROPY: usize = 25;
pub const DENTROPY: usize = 26;
pub const SDATA: usize = 27;
pub const DDATA: usize = 28;
pub const SIAT: usize = 29;
pub const DIAT: usize = 30;
pub const SSTDEV: usize = 31;
pub const DSTDEV: usize = 32;
pub const STCPURG: usize = 33;
pub const DTCPURG: usize = 34;
pub const SSMALLPKTCNT: usize = 35;
pub const DSMALLPKTCNT: usize = 36;
pub const SNONEMPTYPKTCNT: usize = 37;
pub const DNONEMPTYPKTCNT: usize = 38;
pub const SFIRSTNONEMPTYSIZE: usize = 39;
pub const DFIRSTNONEMPTYSIZE: usize = 40;
pub const SSTDEVPAYLOAD: usize = 41;
pub const DSTDEVPAYLOAD: usize = 42;
pub const SMAXPKTSIZE: usize = 43;
pub const DMAXPKTSIZE: usize = 44;
pub const SPD: usize = 45;
pub const REASON: usize = 46;
pub const APPID: usize = 47;
pub const MAX_FIELDS: usize = 48;

pub trait Processor {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn run(&self);

    fn process_stream(&self, input_channel: Receiver<String>, output_channel: SyncSender<String>);

    fn process_loop(
        &self,
        input_spec: String,
        output_spec: String,
        archive_spec: String,
        interval: u32
    ) {
        let (source_channel, input_channel) = mpsc::sync_channel::<String>(8);
        let (output_channel, sink_channel) = mpsc::sync_channel::<String>(8);

        let output_thread = thread::spawn(|| {
            if !output_spec.is_empty() {
                if output_spec.starts_with("tcp://") {
                    sink::socket(output_spec[6..].to_string(), sink_channel);
                } else if output_spec.ends_with("/") {
                    sink::directory(output_spec, archive_spec, interval, sink_channel);
                } else {
                    sink::file(output_spec, sink_channel);
                }
            } else {
                sink::stdout(sink_channel);
            }
        });

        let input_thread = thread::spawn(|| {
            if !input_spec.is_empty() {
                if input_spec.starts_with("tcp://") {
                    source::socket(input_spec[6..].to_string(), source_channel);
                } else if input_spec.ends_with("/") {
                    source::directory(input_spec, archive_spec, interval, source_channel);
                } else {
                    source::file(input_spec, source_channel);
                }
            } else {
                source::stdin(source_channel);
            }
        });
        self.process_stream(input_channel, output_channel);

        input_thread.join().unwrap();
        output_thread.join().unwrap();
    }
}
