type Real = f32;

pub trait Layer {
    fn forward(&self, x: &dyn Layer) -> &dyn Layer;
}

pub struct IdentityLayer {}

impl Layer for IdentityLayer {
    fn forward(&self, x: &dyn Layer) -> &dyn Layer {
        todo!()
    }
}
