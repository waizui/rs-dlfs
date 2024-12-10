type Real = f32;
use nalgebra::DMatrix;

pub fn dot(x: &DMatrix<Real>, y: &DMatrix<Real>) -> DMatrix<Real> {
    assert_eq!(x.ncols(), y.nrows());
    x * y
}

pub fn bdot(x: &DMatrix<Real>, y: &DMatrix<Real>, b: &DMatrix<Real>) -> DMatrix<Real> {
    assert_eq!(x.ncols(), y.nrows());
    assert_eq!(y.ncols(), b.ncols());
    x * y + b
}

pub fn sigmoid(x: &DMatrix<Real>) -> DMatrix<Real> {
    x.map(|e| 1. / (1. + (-e).exp()))
}

pub fn softmax(x: &DMatrix<Real>) -> DMatrix<Real> {
    todo!()
}
