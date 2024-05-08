use anyhow::Result;
use tch::{nn, nn::Module, nn::OptimizerConfig, Kind, Reduction, Tensor};

use crate::flow::Record;

pub trait PyTorchModel {
    fn load(&mut self);
    fn train(&mut self, _record: &Record);
    fn features(&mut self, record: &Record);
    fn score(&mut self, features: &mut [f64; 16]);
}

// TODO: see https://github.com/Stream-AD/MemStream

pub struct MemStreamAutoEncoder {
    pub file_name: String,
    pub valid_model: bool,
}

impl PyTorchModel for MemStreamAutoEncoder {
    fn load(&mut self) {
        self.valid_model = false;
        if !self.file_name.is_empty() {
            // load file
            self.valid_model = true;
        }
    }
    fn features(&mut self, _record: &Record) {
        unimplemented!();
    }
    fn score(&mut self, _features:  &mut [f64; 16]) {
        unimplemented!();
    }
    fn train(&mut self, _record: &Record) {
        unimplemented!();
    }
}
