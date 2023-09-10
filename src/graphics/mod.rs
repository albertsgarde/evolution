mod camera;
pub use camera::Camera;
use macroquad::window;

use crate::world::Vector;

pub fn screen_size() -> Vector {
    Vector::new(window::screen_width(), window::screen_height())
}
