use serde::{Deserialize, Serialize};

use crate::{world::PhysicsBody, Config, State};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Creature {
    energy: f32,
}

impl Creature {
    pub fn new(config: &Config) -> Self {
        Self {
            energy: config.creature_starting_energy(),
        }
    }

    pub fn tick(&self, body: &mut PhysicsBody, state: &State) -> Self {
        const MAX_ACCELERATION: f32 = 4.2;

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
            let norm_acceleration = target_acceleration.normalize() * MAX_ACCELERATION;

            body.accelerate(state.config(), norm_acceleration);
        }

        self.clone()
    }
}