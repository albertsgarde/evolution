use serde::{Deserialize, Serialize};

use crate::world::Location;

mod graphics;
use graphics::Graphics;

const RNG_SEED: u128 = 0;

const WORLD_WIDTH: f32 = 100.0;
const WORLD_HEIGHT: f32 = 100.0;
const UPPER_LEFT: Location = Location::new(0., 0.);
const LOWER_RIGHT: Location = Location::new(WORLD_WIDTH, WORLD_HEIGHT);

const TICK_LENGTH: f32 = 1. / 24.;

const DRAG: f32 = 0.1;

const ENTITY_SIZE: f32 = 2.;

const FOOD_SPAWN_RATE: f32 = 0.4;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub graphics: Graphics,
}

impl Config {
    pub fn rng_seed(&self) -> u128 {
        RNG_SEED
    }

    pub fn world_width(&self) -> f32 {
        WORLD_WIDTH
    }

    pub fn world_height(&self) -> f32 {
        WORLD_HEIGHT
    }

    pub fn upper_left(&self) -> Location {
        UPPER_LEFT
    }

    pub fn lower_right(&self) -> Location {
        LOWER_RIGHT
    }

    pub fn tick_length(&self) -> f32 {
        TICK_LENGTH
    }

    pub fn drag(&self) -> f32 {
        DRAG
    }

    pub fn entity_size(&self) -> f32 {
        ENTITY_SIZE
    }

    pub fn creature_starting_energy(&self) -> f32 {
        40.
    }

    pub fn creature_max_energy(&self) -> f32 {
        100.
    }

    pub fn creature_reproduction_energy(&self) -> f32 {
        60.
    }

    pub fn creature_child_bounce(&self) -> f32 {
        2.
    }

    pub fn food_spawn_rate(&self) -> f32 {
        FOOD_SPAWN_RATE
    }
}
