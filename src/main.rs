use bevy::prelude::*;
mod pet;
mod supplies;
mod utils;
mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(
            Startup,
            (world::make_world, (pet::make_pet, supplies::make_supplies)).chain(),
        )
        .add_systems(
            Update,
            (supplies::move_back, pet::animate_purr, utils::check_supply),
        )
        .run();
}
