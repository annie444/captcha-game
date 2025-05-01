use bevy::prelude::*;
use std::time::Duration;

const PET_COLOR: Color = Color::srgb(255.0, 0.0, 0.0);
const PET_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const PET_RADIUS: f32 = 50.0;
const PET_SCALE: f32 = 6.0;

#[derive(Component, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Pet {
    pub emote: Emote,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default, Debug)]
pub enum Emote {
    #[default]
    Idle,
    Purring,
}

#[derive(Component, Clone, Eq, PartialEq, Default, Debug)]
struct AnimationConfig {
    first: usize,
    last: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first: first,
            last: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

pub fn make_pet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("assets/cat/grey/cat-resting-purr.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 10, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let purring_animation = AnimationConfig::new(1, 10, 10);

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: purring_animation.first,
            }),
            ..Default::default()
        },
        Transform::from_scale(Vec3::splat(PET_SCALE)).with_translation(PET_STARTING_POSITION),
        Pet { emote: Emote::Idle },
        purring_animation,
    ));
}

// This system runs when the user clicks the left arrow key or right arrow key
fn animate_purr(
    mut animation: Single<&mut AnimationConfig, With<Pet>>,
    query: Query<&Pet, With<AnimationConfig>>,
) {
    for pet in query {
        if pet.emote == Emote::Purring {
            animation.frame_timer = AnimationConfig::timer_from_fps(animation.fps);
        }
    }
}
