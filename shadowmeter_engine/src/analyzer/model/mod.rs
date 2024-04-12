
use anyhow::Result;
use tch::{nn, nn::Module, nn::OptimizerConfig, Kind, Reduction, Tensor};


use crate::flow::Record;

pub trait PyTorchModel {
    fn load_model(&mut self);
    fn generate_features(&mut self, record: &Record);
    fn generate_score(&mut self, features: &String);
}


pub struct Example {
    pub file_name: String,
    pub valid_model : bool,
}

impl PyTorchModel for Example {
    fn load_model(&mut self) {
        self.valid_model = false;
        if ! self.file_name.is_empty() {
            // load file
            self.valid_model = true;
        }
      
    }
    fn generate_features(&mut self, _record: &Record) {
        if self.valid_model {

        }
    }
    fn generate_score(&mut self, _features: &String) {
        if self.valid_model {

        }
    }
}
