use bevy::prelude::*;

pub fn make_world(mut commands: Commands) {
    commands.spawn(Camera2d);
}
