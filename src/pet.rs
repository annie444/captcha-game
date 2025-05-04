use bevy::prelude::*;
use std::{fmt, time::Duration};

const PET_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);

#[derive(Component, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default, Debug)]
pub(crate) struct Pet {
    pub(crate) emote: Emote,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Default, Debug)]
pub(crate) enum Emote {
    #[default]
    Idle,
    Purring,
    Eating,
    Drinking,
    Peeing,
    Smelling,
    Playing,
}

impl fmt::Display for Emote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "idle"),
            Self::Purring => write!(f, "purring"),
            Self::Eating => write!(f, "eating"),
            Self::Drinking => write!(f, "drinking"),
            Self::Peeing => write!(f, "peeing"),
            Self::Smelling => write!(f, "smelling"),
            Self::Playing => write!(f, "playing"),
        }
    }
}

#[derive(Component, Clone, Eq, PartialEq, Default, Debug)]
pub(crate) struct AnimationConfig {
    first: usize,
    purr: [usize; 2],
    fps: u8,
    frame_timer: Timer,
    purr_timer: Timer,
}

impl AnimationConfig {
    pub(crate) fn new(first: usize, purr: [usize; 2], fps: u8) -> Self {
        Self {
            first,
            purr,
            fps,
            frame_timer: Self::timer_from_fps(fps),
            purr_timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(
            Duration::from_secs_f32(1.0 / (fps as f32)),
            TimerMode::Repeating,
        )
    }

    pub(crate) fn reset(&mut self) {
        self.frame_timer = Self::timer_from_fps(self.fps);
    }
}

pub(crate) fn make_pet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("cat/grey/cat-resting-purring.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(128), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let purring_animation = AnimationConfig::new(0, [2, 3], 8);

    commands
        .spawn((
            Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: purring_animation.first,
                }),
                ..Default::default()
            },
            Transform::from_translation(PET_STARTING_POSITION),
            Pet { emote: Emote::Idle },
            purring_animation,
            Pickable::default(),
        ))
        .observe(start_petting)
        .observe(petting)
        .observe(stop_petting);
}

fn start_petting(
    trigger: Trigger<Pointer<Over>>,
    mut query: Query<(&mut AnimationConfig, &Sprite), With<Pet>>,
) {
    let mut config = query.get_mut(trigger.target()).unwrap().0;
    if config.purr_timer.paused() {
        config.purr_timer.reset();
        config.purr_timer.unpause();
        config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
    }
}

fn petting(
    trigger: Trigger<Pointer<Move>>,
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Pet, &Sprite)>,
) {
    let (mut config, mut pet, _) = query.get_mut(trigger.target()).unwrap();
    if config.purr_timer.just_finished() && pet.emote != Emote::Purring {
        pet.emote = Emote::Purring;
    } else {
        config.purr_timer.tick(time.delta());
    }
}

fn stop_petting(
    trigger: Trigger<Pointer<Out>>,
    mut query: Query<(&mut AnimationConfig, &mut Pet, &Sprite)>,
) {
    let (mut config, mut pet, _) = query.get_mut(trigger.target()).unwrap();
    config.purr_timer.pause();
    pet.emote = Emote::Idle;
}

pub(crate) fn animate_purr(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite, &Pet)>,
) {
    for (mut config, mut sprite, cat) in &mut query {
        if cat.emote == Emote::Purring {
            config.frame_timer.tick(time.delta());
            if config.frame_timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    if atlas.index == config.purr[0] {
                        atlas.index = config.purr[1];
                        config.reset();
                    } else if atlas.index == config.purr[1] {
                        atlas.index = config.purr[0];
                        config.reset();
                    } else {
                        atlas.index += 1;
                        config.reset();
                    }
                }
            }
        } else if cat.emote == Emote::Idle {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index != config.first {
                    atlas.index -= 1;
                    config.reset();
                }
            }
        }
    }
}
