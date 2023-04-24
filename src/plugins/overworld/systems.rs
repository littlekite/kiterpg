// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{
        Added, Camera2d, Commands, DespawnRecursiveExt, Entity, EventWriter, Name, NextState,
        Query, Res, ResMut, Transform, Vec2, With, Without,Vec3
    },
    window::Window,
};

use crate::plugins::{
    player::{
        components::{
            Player
        },
    },
    atlas::resources::GameAtlases,
    game::states::{GameState, LevelState},
    overworld::states::OverworldState,
    overworld::resources::CurrentRoom,
    tilemap::bundles::TilemapBundle
};

pub fn overworld_setup(
    mut commands: Commands,
    game_atlases: Res<GameAtlases>,
    room: Res<CurrentRoom>,
    mut game_state: ResMut<NextState<GameState>>,
    mut player: Query<&mut Transform, With<Player>>,
    mut next_state: ResMut<NextState<OverworldState>>
) {
    game_state.set(GameState::Running);
    next_state.set(OverworldState::FreeRoam);

    let mut player = player.single_mut();
    player.translation = room.current_player_translation;



}

