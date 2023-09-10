use rand::Rng;
use rand_distr::Distribution;
use rand_pcg::Pcg64Mcg;

use crate::{
    world::{Entity, Location},
    Config,
};

pub struct State {
    config: Config,
    entities: Vec<Entity>,
    tick_count: u64,
    rng: Pcg64Mcg,
}

impl State {
    pub fn init(config: Config, num_creatures: usize) -> Self {
        let mut rng = Pcg64Mcg::new(config.rng_seed());

        let entities = (0..num_creatures)
            .map(|_| {
                Entity::creature(
                    &config,
                    Location::new(
                        rng.gen_range(0.0..config.world_width()),
                        rng.gen_range(0.0..config.world_height()),
                    ),
                )
            })
            .collect();

        Self {
            config,
            entities,
            tick_count: 0,
            rng,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    pub fn entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter()
    }

    fn spawn_food(&mut self) {
        let poisson =
            rand_distr::Poisson::new(self.config.food_spawn_rate() * self.config.tick_length())
                .unwrap(); // Both food spawn rate and tick length should always be positive and non-infinite.
        let num_spawn = poisson.sample(&mut self.rng) as u64;
        for _ in 0..num_spawn {
            self.entities.push(Entity::food(Location::new(
                self.rng.gen_range(0.0..self.config.world_width()),
                self.rng.gen_range(0.0..self.config.world_height()),
            )));
        }
    }

    pub fn tick(&mut self) {
        self.spawn_food();
        let new_entities: Vec<_> = self.entities.iter().map(|e| e.tick(self)).collect();

        // Remove food that is touched by a creature.
        let new_entities = new_entities
            .iter()
            .filter(|entity| {
                !(entity.is_food()
                    && new_entities.iter().any(|other| {
                        other.is_creature()
                            && (entity.location() - other.location()).norm_squared()
                                < self.config().entity_size().powi(2)
                    }))
            })
            .cloned()
            .collect();
        self.entities = new_entities;

        self.tick_count += 1;
    }
}
