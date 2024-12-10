use crate::layers::Layer;
use ndarray::ArrayD;
type Real = f32;

pub struct Network {
    pub layers: Vec<Box<dyn Layer>>,
}

impl Network {
    pub fn predict(&self, x: ArrayD<Real>) -> Option<ArrayD<Real>> {
        if self.layers.is_empty() {
            return None;
        }

        let mut input = x;
        for layer in &self.layers {
            input = layer.forward(&input);
        }

        Some(input)
    }
}
