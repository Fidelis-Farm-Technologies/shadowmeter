
use anyhow::Result;
use tch::{nn, nn::Module, nn::OptimizerConfig, Device};

use std::sync::mpsc::{Receiver, SyncSender};

use crate::flow::Record;

pub struct Analyzer {
    input: Receiver<Record>,
    output: SyncSender<Record>,
    model_file: String,
}

impl Analyzer {
    pub fn new(input: Receiver<Record>, output: SyncSender<Record>, model_file: String) -> Self {
        println!("model file: {}", model_file);
        Self {
            input: input,
            output: output,
            model_file: model_file,
        }
    }

    pub fn process_loop(&mut self)  {
        println!("process_loop: running");
        process_loop
        loop {
            let record = self.input.recv().unwrap();
            self.output.send(record).expect("error sending record");
        }
    }
}
