use rand::Rng;
use rand_distr::Distribution;
use rand_pcg::Pcg64Mcg;

use crate::{
    world::{Entity, Location, Vector},
    Config,
};

pub struct State {
    config: Config,
    entities: Vec<Entity>,
    rng: Pcg64Mcg,
}

impl State {
    pub fn init(config: Config, num_creatures: usize, num_food: usize) -> Self {
        let rng = Pcg64Mcg::new(config.rng_seed());

        /*let mut entities = Vec::with_capacity(num_creatures + num_food);
        entities.extend((0..num_creatures).map(|_| {
            Entity::creature(
                Location::new(
                    rng.gen_range(0.0..config.world_width()),
                    rng.gen_range(0.0..config.world_height()),
                ),
                Vector::new(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0)),
            )
        }));
        entities.extend((0..num_food).map(|_| {
            Entity::food(Location::new(
                rng.gen_range(0.0..config.world_width()),
                rng.gen_range(0.0..config.world_height()),
            ))
        }));*/

        let creature = Entity::creature(Location::new(50., 50.), Vector::new(20., -5.));
        let food = Entity::food(Location::new(35., 20.));
        let entities = vec![creature, food];

        Self {
            config,
            entities,
            rng,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
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
        let new_entities = self.entities.iter().map(|e| e.tick(self)).collect();
        self.entities = new_entities;
    }
}
