// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{
        Added, Camera2d, Commands, DespawnRecursiveExt, Entity, EventWriter, Name, NextState,
        Query, Res, ResMut, Transform, Vec2, With, Without,
    },
    window::Window,
};

use crate::plugins::{
    atlas::resources::GameAtlases,
    game::states::{GameState, LevelState},
    tilemap::bundles::TilemapBundle,
};

pub fn overworld_setup(
    mut commands: Commands,
    game_atlases: Res<GameAtlases>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::Running);

    commands
        .spawn(TilemapBundle::new(
            game_atlases.tileset.clone(),
            (16.0, 16.0).into(),
            (15, 20).into(),
            vec![
                vec![16, 15, 15, 16,15, 16, 15, 16,15,15, 16, 15, 15, 15, 16],
                vec![11, 10, 11, 12,12,11,10,10, 10,10, 10, 12, 11, 11, 10],
                vec![12, 15, 4, 9, 9,4,4,4,4,4, 4, 4, 4, 4, 10],
                vec![12, 10, 4, 4, 4,4, 4,4,4,4,14, 25, 4, 4, 11],
                vec![12, 4, 9, 9, 4,4, 4,4,4,4,4, 9, 4, 12, 12],
                vec![10, 4, 4, 4, 4, 4,4,4, 4,4,4,4, 4, 4, 11],
                vec![12, 9, 4, 4, 19,4,4, 4,4,4, 4,9, 26, 4, 12],
                vec![12, 4, 14, 4, 4,4, 4,4, 4,4,4,4, 27, 28, 11],
                vec![12, 4, 4, 4, 4, 4,4,4,4,4, 4,4, 4, 4, 10],
                vec![11, 25, 4, 14, 4,14,4,4, 4,4,4, 4, 9, 4, 12],
                vec![12, 4, 4, 9, 9,4, 4,4, 4,4,4,4, 4, 4, 10],
                vec![12, 4, 9, 4, 4, 4,4,4, 4,14,4,4, 19, 4, 11],
                vec![12, 4, 29, 4, 4, 4,4,4, 4,4,4,4, 4, 4, 12],
                vec![11, 9, 4, 22, 22, 4,4,4,4, 4,4, 4,4, 4, 10],
                vec![10, 12, 9, 20, 21, 4,4,4, 4,4,25,4, 4, 9, 10],
                vec![12, 4, 4, 4, 4, 25, 4,4,4,4,4, 9,4, 4, 11],
                vec![12, 4, 9, 4, 4, 4, 4,14,4,4, 4,4,4, 4, 11],
                vec![12, 4, 14, 4, 9, 4,4, 4, 4,4,9,4, 4,4, 12],
                vec![10, 15, 16, 15, 9, 9,9, 9,15,15,9,9, 15, 16, 10],
                vec![12, 10, 11, 12, 23, 24,11, 12,23, 24,11,11,11, 12, 10],
            ],
            vec![0, 1, 2, 3, 5, 6, 7, 8, 10, 11, 12, 20, 21, 27, 28],
            None,
        ))
        .insert(Name::from("OverworldTilemap"));
}

