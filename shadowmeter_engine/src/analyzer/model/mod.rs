
use anyhow::Result;
use tch::{nn, nn::Module, nn::OptimizerConfig, Kind, Reduction, Tensor};


use crate::flow::Record;

pub trait PyTorchModel {
    fn load(&mut self);
    fn features(&mut self, record: &Record);
    fn predict(&mut self, features: &String);
}


pub struct Example {
    pub file_name: String,
    pub valid_model : bool,
}

impl PyTorchModel for Example {
    fn load(&mut self) {
        self.valid_model = false;
        if ! self.file_name.is_empty() {
            // load file
            self.valid_model = true;
        }
      
    }
    fn features(&mut self, _record: &Record) {
        if self.valid_model {

        }
    }
    fn predict(&mut self, _features: &String) {
        if self.valid_model {

        }
    }
}
