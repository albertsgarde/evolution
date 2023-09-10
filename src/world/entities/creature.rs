use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

use crate::{world::PhysicsBody, Config, State};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Creature {
    energy: f32,
    max_acceleration: f32,
}

impl Creature {
    pub fn new(config: &Config) -> Self {
        Self {
            energy: config.creature_starting_energy(),
            max_acceleration: 4.,
        }
    }

    pub fn energy(&self) -> f32 {
        self.energy
    }

    pub fn max_acceleration(&self) -> f32 {
        self.max_acceleration
    }

    pub fn tick(&self, body: &mut PhysicsBody, state: &State) -> Self {
        if let Some(food) =
            state
                .entities()
                .filter(|&entity| entity.is_food())
                .min_by(|&entity1, &entity2| {
                    (entity1.location() - body.location())
                        .norm_squared()
                        .total_cmp(&(entity2.location() - body.location()).norm_squared())
                })
        {
            let target_location = food.location();

            let target_delta = target_location - body.location();
            let cur_velocity = body.velocity();
            let target_acceleration = target_delta - cur_velocity;
            let norm_acceleration = target_acceleration.normalize() * self.max_acceleration;

            body.accelerate(state.config(), norm_acceleration);
        }

        Self {
            energy: self.energy - 0.025 * self.max_acceleration * state.config().tick_length(),
            ..self.clone()
        }
    }

    pub fn eat(&self, config: &Config, energy: f32) -> Self {
        let max_energy = config.creature_max_energy();
        Self {
            energy: self.energy + energy * (1. - (self.energy / max_energy).powi(2)),
            ..self.clone()
        }
    }

    pub fn reproduce(&self, config: &Config, rng: &mut impl Rng) -> Option<(Self, Self)> {
        if self.energy > config.creature_reproduction_energy() {
            let child_energy = self.energy / 2.;
            let log_normal = Normal::new(0., 0.1).unwrap().map(|x: f32| x.exp());
            Some((
                Self {
                    energy: child_energy,
                    max_acceleration: self.max_acceleration * rng.sample(&log_normal),
                },
                Self {
                    energy: child_energy,
                    max_acceleration: self.max_acceleration * rng.sample(log_normal),
                },
            ))
        } else {
            None
        }
    }
}
