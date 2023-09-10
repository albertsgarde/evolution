use crate::{Config, State};

use super::{Location, Vector};

#[derive(Clone, Debug)]
pub struct PhysicsBody {
    location: Location,
    velocity: Vector,
}

impl PhysicsBody {
    pub fn new(location: Location, velocity: Vector) -> Self {
        Self { location, velocity }
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn velocity(&self) -> Vector {
        self.velocity
    }

    pub fn add_velocity(self, velocity: Vector) -> Self {
        PhysicsBody {
            velocity: self.velocity + velocity,
            ..self
        }
    }

    /// Add a acceleration vector to the bodies velocity.
    pub fn accelerate(&mut self, config: &Config, acceleration: Vector) {
        self.velocity += acceleration * config.tick_length();
    }

    pub fn tick(&self, state: &State) -> Self {
        let config = state.config();
        let location = (self.location + self.velocity * config.tick_length())
            .clamp(config.upper_left(), config.lower_right());
        let velocity = self.velocity * (-config.drag() * config.tick_length()).exp();

        Self { location, velocity }
    }
}
