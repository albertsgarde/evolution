use rand::Rng;
use rand_distr::Distribution;
use rand_pcg::Pcg64Mcg;

use crate::{
    world::{Entity, EntityData, Location},
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

    pub fn entities(&self) -> impl ExactSizeIterator<Item = &Entity> {
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
        // Tick entities.
        let new_entities: Vec<_> = self.entities.iter().map(|e| e.tick(self)).collect();

        // Eat food.
        let mut energy_eaten = vec![(true, 0.); new_entities.len()];
        for (food_index, food) in new_entities
            .iter()
            .enumerate()
            .filter(|(_, entity)| entity.is_food())
        {
            if let Some(creature_index) = new_entities.iter().position(|other| {
                other.is_creature()
                    && (food.location() - other.location()).norm_squared()
                        < self.config().entity_size().powi(2)
            }) {
                energy_eaten[creature_index].1 += 3.;
                energy_eaten[food_index].0 = false;
            }
        }
        // Feed creatures.
        let new_entities: Vec<_> = new_entities
            .into_iter()
            .zip(energy_eaten)
            .filter_map(|(entity, (survived, energy))| {
                if survived {
                    Some(if energy != 0. {
                        entity.eat(&self.config, energy)
                    } else {
                        entity
                    })
                } else {
                    None
                }
            })
            // Kill creatures with no energy.
            .filter(|entity| {
                if let EntityData::Creature(creature) = entity.entity_data() {
                    creature.energy() > 0.
                } else {
                    true
                }
            })
            // Reproduce.
            .flat_map(|entity| entity.reproduce(&self.config, &mut self.rng))
            .collect();
        self.entities = new_entities;

        self.tick_count += 1;
    }
}
