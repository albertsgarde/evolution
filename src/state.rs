use rand::Rng;

use crate::{
    world::{Entity, Location, Vector},
    Config,
};

pub struct State {
    config: Config,
    entities: Vec<Entity>,
}

impl State {
    pub fn init(config: Config, num_creatures: usize, num_food: usize, rng: &mut impl Rng) -> Self {
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

        Self { config, entities }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter()
    }

    pub fn tick(&mut self) {
        let new_entities = self.entities.iter().map(|e| e.tick(self)).collect();
        self.entities = new_entities;
    }
}
