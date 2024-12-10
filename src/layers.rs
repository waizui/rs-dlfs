type Real = f32;
use crate::functions::*;
use nalgebra::DMatrix;

pub trait Layer {
    fn forward(&self, x: &DMatrix<Real>) -> DMatrix<Real>;
}

#[derive(Debug)]
pub struct IdentityLayer {}

impl Layer for IdentityLayer {
    fn forward(&self, x: &DMatrix<Real>) -> DMatrix<Real> {
        x.clone()
    }
}

#[derive(Debug)]
pub struct SigmoidLayer {
    pub w: DMatrix<Real>,
    pub b: DMatrix<Real>,
}

impl Layer for SigmoidLayer {
    fn forward(&self, x: &DMatrix<Real>) -> DMatrix<Real> {
        let r = bdot(x, &self.w, &self.b);
        crate::functions::sigmoid(&r)
    }
}
