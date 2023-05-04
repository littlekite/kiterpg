// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::*;
use bevy::prelude::{in_state, IntoSystemConfig, OnUpdate, Plugin};


use crate::plugins::{
    overworld::states::OverworldState,
    overworld::{CombatDescriptor,CombatStartTag},
    player::systems::animation,
    game::states::GameState
};
use crate::{
    assets::GameAssets,
    plugins::{
        camera::{bundles::GameCameraBundle, components::GameCameraState},
    },
    states::AppState,
};



pub mod systems;
use self::components::Icon;
use self::{
    systems::{textfun,spawn_player_attack_icons}
};

use super::combat::states::CombatState;
pub mod components;

pub struct FightPlugin;

#[derive(Bundle)]
pub struct IconBundle {
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    icon: Icon,
}

impl Default for IconBundle {
    fn default() -> Self {
        Self {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::splat(1.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 850.)),
                ..Default::default()
            },
            icon: Icon::Pointer,
        }
    }
}

impl IconBundle {
    pub fn new(position: Vec2, icon: Icon, scale: Vec2) -> Self {
        let mut bundle = IconBundle { icon, ..default() };

        bundle.sprite_sheet.transform.translation = position.extend(850.);
        bundle.sprite_sheet.transform.scale = Vec3::new(10., 10., 1.);

        bundle
    }
}

#[derive(Component, Default, Reflect)]
pub struct CurrentSelectedMenuItem {
    pub selection: i32,
    pub slots: i32,
}

#[derive(Component, Reflect)]
pub struct SelectionIcon;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn_player_attack_icons.in_schedule(OnEnter(CombatState::PlayerSelecting)),
        ).add_systems(
            (textfun,).in_set(OnUpdate(AppState::InGame))
            .distributive_run_if(in_state(GameState::Combat))
        );
    }
}




