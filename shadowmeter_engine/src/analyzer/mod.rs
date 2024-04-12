
mod model;

use anyhow::Result;
use tch::{nn, nn::Module, nn::OptimizerConfig, Kind, Reduction, Tensor};

use std::sync::mpsc::{Receiver, SyncSender};

use crate::flow::Record;
use crate::analyzer::model::Example;

pub struct Analyzer {
    input: Receiver<Record>,
    output: SyncSender<Record>,
    model_file: String,
}

impl Analyzer {
    pub fn new(input: Receiver<Record>, output: SyncSender<Record>, model_file: &String) -> Self {
        println!("model file: {}", model_file);
        let _model = Example {
            file_name : model_file.clone(),
            valid_model: false
        };
        Self {
            input: input,
            output: output,
            model_file: model_file.clone(),
        }
 
    }

    pub fn process_loop(&mut self)  {
        println!("process_loop: running");

        loop {            
            let record = self.input.recv().unwrap();
            self.output.send(record).expect("error sending record");
        }
    }
}
