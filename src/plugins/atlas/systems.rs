// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Assets, Commands, NextState, Res, ResMut, Vec2},
    sprite::TextureAtlas,
};

use crate::{assets::GameAssets, plugins::atlas::resources::GameAtlases, states::AppState};

pub(super) fn build_atlases(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let texture_atlas_player = TextureAtlas::from_grid(
        game_assets.image_player.clone(),
        Vec2::new(32.0, 32.0),
        4,
        9,
        None,
        None,
    );

    let texture_atlas_tileset = TextureAtlas::from_grid(
        game_assets.image_tileset.clone(),
        Vec2::new(16.0, 16.0),
        5,
        6,
        None,
        None,
    );

    commands.insert_resource(GameAtlases {
        player: texture_atlases.add(texture_atlas_player),
        tileset: texture_atlases.add(texture_atlas_tileset),

    });
    println!("aaa");
    next_state.set(AppState::Introduction);
}
