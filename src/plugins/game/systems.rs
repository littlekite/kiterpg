// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{
        Commands, DespawnRecursiveExt, Entity, EventReader, Input, KeyCode, NextState, Query, Res,
        ResMut, Vec2, Without,
    },
    window::{Window, WindowFocused},
};

use crate::plugins::{
    camera::{bundles::GameCameraBundle, components::GameCameraState},
    atlas::resources::GameAtlases,
};

use super::{
    states::{GameState, LevelState},
};

pub fn game_setup(
    mut commands: Commands,
    game_atlases: Res<GameAtlases>,
    mut game_state: ResMut<NextState<GameState>>,
    mut level_state: ResMut<NextState<LevelState>>
) {

    commands.spawn(GameCameraBundle::new(0.2, GameCameraState::FollowPlayer));
   
    game_state.set(GameState::Running);
    level_state.set(LevelState::Overworld);
}

