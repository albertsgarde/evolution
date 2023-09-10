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
    entity_type: EntityData,
}

impl Entity {
    pub fn food(location: Location) -> Self {
        Self {
            body: PhysicsBody::new(location, Vector::new(0.0, 0.0)),
            entity_type: EntityData::food(),
        }
    }

    pub fn creature(config: &Config, location: Location) -> Self {
        Self {
            body: PhysicsBody::new(location, Vector::new(0.0, 0.0)),
            entity_type: EntityData::creature(config),
        }
    }

    pub fn entity_data(&self) -> &EntityData {
        &self.entity_type
    }

    pub fn entity_type(&self) -> EntityType {
        self.entity_type.entity_type()
    }

    pub fn location(&self) -> Location {
        self.body.location()
    }

    pub fn is_food(&self) -> bool {
        matches!(self.entity_type, EntityData::Food)
    }

    pub fn is_creature(&self) -> bool {
        matches!(self.entity_type, EntityData::Creature(_))
    }

    pub fn tick(&self, state: &State) -> Self {
        let mut body = self.body.clone();
        let entity_type = self.entity_type.tick(&mut body, state);
        let body = body.tick(state);
        Self { body, entity_type }
    }

    pub fn eat(&self, config: &Config, energy: f32) -> Self {
        match &self.entity_type {
            EntityData::Creature(creature) => Self {
                body: self.body.clone(),
                entity_type: EntityData::Creature(creature.eat(config, energy)),
            },
            EntityData::Food => panic!("Food cannot eat!"),
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
