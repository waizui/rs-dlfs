use crate::{functions::bdot, layers::*, util};
use nalgebra::DMatrix;
type Real = f32;

pub struct Network {
    pub layers: Vec<Box<dyn Layer>>,
}

impl Network {
    pub fn predict(&self, x: DMatrix<Real>) -> Option<DMatrix<Real>> {
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

#[test]
fn test_network() {
    let mut net = Network { layers: Vec::new() };

    net.layers.push(Box::new(SigmoidLayer {
        w: DMatrix::from_row_slice(2, 3, &[0.1, 0.3, 0.5, 0.2, 0.4, 0.6]),
        b: DMatrix::from_row_slice(1, 3, &[0.1, 0.2, 0.3]),
    }));

    net.layers.push(Box::new(SigmoidLayer {
        w: DMatrix::from_row_slice(3, 2, &[0.1, 0.4, 0.2, 0.5, 0.3, 0.6]),
        b: DMatrix::from_row_slice(1, 2, &[0.1, 0.2]),
    }));

    net.layers.push(Box::new(IdentityLayer {}));

    if let Some(res) = net.predict(DMatrix::from_row_slice(1, 2, &[1.0, 0.5])) {
        let w = DMatrix::from_row_slice(2, 2, &[0.1, 0.3, 0.2, 0.4]);
        let b = DMatrix::from_row_slice(1, 2, &[0.1, 0.2]);
        let r = bdot(&res, &w, &b);
        assert_eq!(r[0], 0.3168271);
        assert_eq!(r[1], 0.6962791);
    } else {
        panic!("error");
    }
}
