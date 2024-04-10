
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
        Self {
            input: input,
            output: output,
            model_file: model_file,
        }
    }

    pub fn process_loop(&mut self)  {
        println!("model file: {}", self.model_file);

        loop {
            let record = self.input.recv().unwrap();
            self.output.send(record).expect("error sending record");
        }
    }
}
