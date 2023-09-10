use serde::{Deserialize, Serialize};
use strum::EnumDiscriminants;

use crate::{
    world::{Location, Vector},
    State,
};

use super::PhysicsBody;

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

    pub fn creature(location: Location, velocity: Vector) -> Self {
        Self {
            body: PhysicsBody::new(location, velocity),
            entity_type: EntityData::creature(),
        }
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
        matches!(self.entity_type, EntityData::Creature)
    }

    pub fn tick(&self, state: &State) -> Self {
        let mut body = self.body.clone();
        let entity_type = self.entity_type.tick(&mut body, state);
        let body = body.tick(state);
        Self { body, entity_type }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumDiscriminants)]
#[strum_discriminants(derive(Serialize, Deserialize))]
#[strum_discriminants(vis(pub))]
#[strum_discriminants(name(EntityType))]
enum EntityData {
    Creature,
    Food,
}

impl EntityData {
    pub fn food() -> EntityData {
        EntityData::Food
    }

    pub fn creature() -> EntityData {
        EntityData::Creature
    }

    pub fn entity_type(&self) -> EntityType {
        match self {
            EntityData::Creature => EntityType::Creature,
            EntityData::Food => EntityType::Food,
        }
    }

    pub fn tick(&self, body: &mut PhysicsBody, state: &State) -> Self {
        match self {
            EntityData::Creature => {
                const MAX_ACCELERATION: f32 = 4.2;

                if let Some(food) = state.entities().filter(|&entity| entity.is_food()).min_by(
                    |&entity1, &entity2| {
                        (entity1.location() - body.location())
                            .norm_squared()
                            .total_cmp(&(entity2.location() - body.location()).norm_squared())
                    },
                ) {
                    let target_location = food.location();

                    let target_delta = target_location - body.location();
                    let cur_velocity = body.velocity();
                    let target_acceleration = target_delta - cur_velocity;
                    let norm_acceleration = target_acceleration.normalize() * MAX_ACCELERATION;

                    body.accelerate(state.config(), norm_acceleration);
                }

                *self
            }
            _ => *self,
        }
    }
}
