use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

use crate::pet::{Emote, Pet};
use crate::supplies::{Supplies, Supply};

impl From<&Supplies> for Emote {
    fn from(value: &Supplies) -> Self {
        match value {
            Supplies(Supply::Toy) => Emote::Playing,
            Supplies(Supply::LitterScoop) => Emote::Smelling,
            Supplies(Supply::Litter) => Emote::Peeing,
            Supplies(Supply::Milk) => Emote::Drinking,
            Supplies(Supply::Food) => Emote::Eating,
        }
    }
}

pub(crate) fn check_supply(
    mut pet_query: Single<(&Transform, &mut Pet)>,
    supply_query: Query<(&Transform, &Supplies)>,
) {
    let pet_loc = pet_query.0;
    let pet = &mut pet_query.1;
    let pet_box = Aabb2d::new(pet_loc.translation.truncate(), Vec2::new(64.0, 64.0));
    for (supply_loc, supply) in supply_query {
        let supply_box = Aabb2d::new(
            supply_loc.translation.truncate(),
            supply_loc.scale.truncate() / 2.0,
        );
        if supply_box.intersects(&pet_box) {
            pet.emote = supply.into();
        }
    }
}



pub(crate) fn 
