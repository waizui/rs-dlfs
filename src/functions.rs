type Real = f32;
use ndarray::ArrayD;

pub fn sigmoid(x: &ArrayD<Real>) -> ArrayD<Real> {
    1. / (1. + (-x).exp())
}

pub fn softmax(x: &ArrayD<Real>) -> ArrayD<Real> {
    todo!()
}
