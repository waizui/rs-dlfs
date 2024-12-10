use crate::layers::Layer;

pub struct Network {
    pub layers: Vec<Box<dyn Layer>>,
}

impl Network {
    pub fn forward(&self, x: &dyn Layer) -> Option<Box<dyn Layer>> {
        if self.layers.is_empty() {
            return None;
        }

        let mut input = x;
        for layer in &self.layers {
            input = layer.forward(input);
        }

        todo!()
        // Some(layer.clone())
    }
}
