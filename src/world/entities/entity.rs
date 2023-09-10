use std::f32::consts::PI;

use itertools::Either;
use rand::Rng;
use serde::{Deserialize, Serialize};
use strum::EnumDiscriminants;

use crate::{
    world::{Location, PhysicsBody, Vector},
    Config, State,
};

use super::creature::Creature;

#[derive(Debug, Clone)]
pub struct Entity {
    body: PhysicsBody,
    data: EntityData,
}

impl Entity {
    pub fn food(location: Location) -> Self {
        Self {
            body: PhysicsBody::new(location, Vector::new(0.0, 0.0)),
            data: EntityData::food(),
        }
    }

    pub fn creature(config: &Config, location: Location) -> Self {
        Self {
            body: PhysicsBody::new(location, Vector::new(0.0, 0.0)),
            data: EntityData::creature(config),
        }
    }

    pub fn entity_data(&self) -> &EntityData {
        &self.data
    }

    pub fn entity_type(&self) -> EntityType {
        self.data.entity_type()
    }

    pub fn location(&self) -> Location {
        self.body.location()
    }

    pub fn is_food(&self) -> bool {
        matches!(self.data, EntityData::Food)
    }

    pub fn is_creature(&self) -> bool {
        matches!(self.data, EntityData::Creature(_))
    }

    pub fn tick(&self, state: &State) -> Self {
        let mut body = self.body.clone();
        let entity_type = self.data.tick(&mut body, state);
        let body = body.tick(state);
        Self {
            body,
            data: entity_type,
        }
    }

    pub fn eat(&self, config: &Config, energy: f32) -> Self {
        match &self.data {
            EntityData::Creature(creature) => Self {
                body: self.body.clone(),
                data: EntityData::Creature(creature.eat(config, energy)),
            },
            EntityData::Food => panic!("Food cannot eat!"),
        }
    }

    pub fn reproduce(self, config: &Config, rng: &mut impl Rng) -> impl Iterator<Item = Self> {
        match &self.data {
            EntityData::Creature(creature) => {
                if let Some((child1, child2)) = creature.reproduce(config, rng) {
                    let child_bounce = config.creature_child_bounce()
                        * Vector::new(rng.gen_range(-PI..PI).cos(), rng.gen_range(-PI..PI).sin());
                    Either::Left(
                        [
                            Self {
                                body: self.body.clone().add_velocity(child_bounce),
                                data: EntityData::Creature(child1),
                            },
                            Self {
                                body: self.body.add_velocity(-child_bounce),
                                data: EntityData::Creature(child2),
                            },
                        ]
                        .into_iter(),
                    )
                } else {
                    Either::Right(std::iter::once(self))
                }
            }
            EntityData::Food => Either::Right(std::iter::once(self)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, EnumDiscriminants)]
#[strum_discriminants(derive(Serialize, Deserialize))]
#[strum_discriminants(vis(pub))]
#[strum_discriminants(name(EntityType))]
pub enum EntityData {
    Creature(Creature),
    Food,
}

impl EntityData {
    pub fn food() -> EntityData {
        EntityData::Food
    }

    pub fn creature(config: &Config) -> EntityData {
        EntityData::Creature(Creature::new(config))
    }

    pub fn entity_type(&self) -> EntityType {
        match self {
            EntityData::Creature(_) => EntityType::Creature,
            EntityData::Food => EntityType::Food,
        }
    }

    pub fn tick(&self, body: &mut PhysicsBody, state: &State) -> Self {
        match self {
            EntityData::Creature(creature) => EntityData::Creature(creature.tick(body, state)),
            _ => self.clone(),
        }
    }
}
