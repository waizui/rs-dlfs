type Real = f32;

use ndarray::ArrayD;

pub trait Layer {
    fn forward(&self, x: &ArrayD<Real>) -> ArrayD<Real>;
}

pub struct IdentityLayer {}

impl Layer for IdentityLayer {
    fn forward(&self, x: &ArrayD<Real>) -> ArrayD<Real> {
        x.clone()
    }
}

pub struct SigmoidLayer {}

impl Layer for SigmoidLayer {
    fn forward(&self, x: &ArrayD<Real>) -> ArrayD<Real> {
        crate::functions::sigmoid(x)
    }
}
