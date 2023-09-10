use nalgebra::Vector4;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Graphics {}

impl Graphics {
    pub fn creature_color(&self) -> Vector4<f32> {
        Vector4::new(1., 0., 0., 1.)
    }
}
