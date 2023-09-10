use macroquad::{camera::Camera2D, math::Rect};

use crate::{
    world::{Location, Vector},
    Config,
};

pub struct Camera {
    /// The location of the upper left of the camera.
    location: Location,
    zoom: f32,
}

impl Camera {
    pub fn new(location: Location, zoom: f32) -> Self {
        Self { location, zoom }
    }

    pub fn view_whole_world(config: &Config, screen_size: Vector) -> Self {
        let min_world_screen_ratio =
            (screen_size.x / config.world_width()).min(screen_size.y / config.world_height());
        Self::new(config.upper_left(), min_world_screen_ratio)
    }

    /// Returns a `macroquad` [`Camera2D`](macroquad::camera::Camera2D) that matches the current camera.
    pub fn mq_camera(&self, screen_size: Vector) -> Camera2D {
        let world_screen_size = screen_size / self.zoom;

        Camera2D::from_display_rect(Rect::new(
            self.location.x(),
            self.location.y() + world_screen_size.y,
            world_screen_size.x,
            -world_screen_size.y,
        ))
    }

    pub fn world_to_camera(&self, location: Location) -> Location {
        Location::ORIGIN + (location - self.location) * self.zoom
    }

    pub fn camera_to_world(&self, location: Location) -> Location {
        self.location + (location - Location::ORIGIN) / self.zoom
    }
}
