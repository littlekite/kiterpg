// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{
        Commands, DespawnRecursiveExt, Entity, EventReader, Name, Input, KeyCode, NextState, Query, Res,
        ResMut, Vec2,Vec3, Without, AssetServer, Assets, Resource, Handle,
    },
    window::{Window, WindowFocused}, sprite::{TextureAtlas, TextureAtlasSprite}, utils::HashMap,
};

use crate::plugins::{
    camera::{bundles::GameCameraBundle, components::GameCameraState},
    atlas::resources::GameAtlases,
    player::bundles::PlayerBundle,
    tilemap::bundles::TilemapBundle,
    overworld::resources::CurrentRoom, fight::{components::Icon, Weapon}
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

    let room = CurrentRoom {
        current_player_translation: Vec3::new(112.0, 100.0, 10.)
    };
    commands.insert_resource(room);

    commands.spawn(GameCameraBundle::new(0.2, GameCameraState::FollowPlayer));
    commands.spawn(PlayerBundle::new(
        game_atlases.player.clone(),
        Vec2 { x: 32.0, y: 32.0 },
        Vec2 { x: 112.0, y: 100.0 },
        15.0,
        65.0,
        18.0,
        10.0,
        90.0,
        150.0,
        0.2,
    ));
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
    game_state.set(GameState::Running);
    level_state.set(LevelState::Overworld);
}

#[derive(Resource)]
pub struct SpriteSheetMaps {
    icon_atlas: Handle<TextureAtlas>,
    pub icons: HashMap<Icon, usize>,
    pub weapons: HashMap<Weapon, usize>,
}

pub fn setup_spritesheet_maps(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let texture_handle = asset_server.load("input_icons/Tilemap/tilemap.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        34,
        24,
        Some(Vec2::splat(1.0)),
        None,
    );
    
    let icon_atlas = texture_atlases.add(texture_atlas);
    let icons = HashMap::from([
        (Icon::Pointer, 34 * 17),
        (Icon::KeyE, 19 + 34 * 10),
    ]);
    let weapons = HashMap::from([(Weapon::BasicStaffOrange, 42), (Weapon::BasicSpear, 47)]);

    commands.insert_resource(SpriteSheetMaps {
        icon_atlas,
        weapons,
        icons
    });
}

pub fn update_art(
    mut icons: Query<
    (&mut TextureAtlasSprite, &mut Handle<TextureAtlas>, &Icon),Without<Weapon>>,
    mut weapons: Query<
    (&mut TextureAtlasSprite, &mut Handle<TextureAtlas>, &Weapon),Without<Icon>>,
    sprite_sheets: Res<SpriteSheetMaps>,
){
    for (mut sprite, mut atlas, icon) in &mut icons {
        *atlas = sprite_sheets.icon_atlas.clone();
        sprite.index = sprite_sheets.icons[icon];
    }
    for (mut sprite, mut atlas, weapon) in &mut weapons {
        *atlas = sprite_sheets.icon_atlas.clone();
        sprite.index = sprite_sheets.weapons[weapon];
    }
}