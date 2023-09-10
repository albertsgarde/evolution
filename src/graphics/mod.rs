mod camera;
pub use camera::Camera;
use macroquad::{color::Color, window};
use nalgebra::Vector4;

use crate::world::Vector;

pub fn vec_to_color(vec: Vector4<f32>) -> Color {
    Color::new(vec.x, vec.y, vec.z, vec.w)
}

pub fn screen_size() -> Vector {
    Vector::new(window::screen_width(), window::screen_height())
}
