// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Bundle, Handle, Transform, Vec2},
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::Timer,
    utils::default,
};

use crate::plugins::{
    animation::{bundles::AnimationBundle, components::AnimationClip},
};

use super::{
    components::{
        Spawner, SpawnerDeathTimer, SpawnerHealth, SpawnerHealthMax, SpawnerIsStunnedTimer,
        SpawnerName, SpawnerTimer,
    },
    states::SpawnerState,
};

#[derive(Bundle)]
pub struct SpawnerBundle {
    tag: Spawner,
    name: SpawnerName,
    spawn_timer: SpawnerTimer,
    state: SpawnerState,
    stunned_timer: SpawnerIsStunnedTimer,
    death_timer: SpawnerDeathTimer
}

impl SpawnerBundle {
    pub fn new(
        name: &str,
        seconds_between: f32,
        seconds_stunned: f32,
    ) -> Self {
        let mut spawn_timer = SpawnerTimer(Timer::from_seconds(
            seconds_between,
            bevy::time::TimerMode::Repeating,
        ));
        spawn_timer.0.set_elapsed(spawn_timer.0.duration());

        let stunned_timer = SpawnerIsStunnedTimer(Timer::from_seconds(
            seconds_stunned,
            bevy::time::TimerMode::Once,
        ));
        let death_timer = SpawnerDeathTimer(Timer::from_seconds(0.5, bevy::time::TimerMode::Once));

        SpawnerBundle {
            tag: Spawner,
            name: SpawnerName(name.into()),
            spawn_timer,
            state: SpawnerState::default(),
            stunned_timer,
            death_timer
        }
    }
}
