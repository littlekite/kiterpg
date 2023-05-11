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
use self::systems::{player_select_attack, update_icon_location, attack_flow, spawn_player_attack, lock_in_attack, player_action_timing, spawn_enemy_attack};
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

//XXX where does weapon declaration belong
#[derive(Component, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum Weapon {
    #[default]
    BasicStaffOrange,
    BasicSpear,
}

#[derive(Bundle)]
pub struct WeaponBundle {
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    weapon: Weapon,
}

impl Default for WeaponBundle {
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
            weapon: Weapon::BasicSpear,
        }
    }
}

impl WeaponBundle {
    pub fn new(position: Vec2, weapon: Weapon, scale: Vec2) -> Self {
        let mut bundle = WeaponBundle {
            weapon,
            ..default()
        };

        bundle.sprite_sheet.transform.translation = position.extend(850.);
        bundle.sprite_sheet.transform.scale = Vec3::new(10., 10., 1.);

        bundle
    }
}

#[derive(Component, Reflect)]
pub struct WeaponIcon(pub i32);

#[derive(PartialEq, Eq, Reflect)]
pub enum WeaponAttackType {
    Melee,
    Range,
}

#[derive(Component, Reflect)]
pub struct AttackAnimation {
    pub starting_x: f32,
    pub starting_y: f32,
    pub ending_x: f32,
    pub ending_y: f32,
    pub max_weapon_rotation: f32,
}
#[derive(Reflect, PartialEq, Eq)]
pub enum AttackStage {
    Charge,
    WalkUp,
    Action,
    CoolDown,
}

#[derive(Reflect, Copy, Clone, PartialEq, Eq, Debug)]
pub enum ActionTiming {
    NotEntered,
    Early,
    Critical,
    Late,
}
pub struct Action {
    //TODO action type
    pub stage: AttackStage,
    pub action_input: ActionTiming,
}

#[derive(Component)]
pub struct Attack {
    pub attacker: Entity,
    pub target: Entity,
    pub attack_type: WeaponAttackType,
    pub current_stage: usize,
    pub stages: Vec<(AttackStage, f32)>,
    pub action: Action,
    pub timer: Timer,
}

#[derive(Bundle)]
pub struct AttackBundle {
    attack: Attack,
    animation: AttackAnimation,
}

pub fn despawn_with<T: Component>(mut commands: Commands, matches: Query<Entity, With<T>>) {
    for entity in &matches {
        commands.entity(entity).despawn_recursive();
    }
}



impl Plugin for FightPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(despawn_with::<Attack>.in_schedule(OnExit(CombatState::PlayerAttacking)))
        .add_system(despawn_with::<Attack>.in_schedule(OnExit(CombatState::EnemyAttacking)))
        .add_system(
            spawn_player_attack_icons.in_schedule(OnEnter(CombatState::PlayerSelecting)),
        ).add_systems(
            (textfun,).in_set(OnUpdate(AppState::InGame))
            .distributive_run_if(in_state(GameState::Combat))
        ).add_systems(
            (player_select_attack,update_icon_location)
                .in_set(OnUpdate(CombatState::PlayerSelecting)),
        ).add_system(
            attack_flow
                .in_set(OnUpdate(CombatState::PlayerAttacking)),
        ).add_system(
            attack_flow
                .in_set(OnUpdate(CombatState::EnemyAttacking)),
        ).add_systems(
            (
                lock_in_attack,
            )
                .chain()
                .in_schedule(OnExit(CombatState::PlayerSelecting)),
        ).add_system(spawn_player_attack.in_schedule(OnEnter(CombatState::PlayerAttacking)))
         .add_system(spawn_enemy_attack.in_schedule(OnEnter(CombatState::EnemyAttacking)))
        .add_systems(
            (player_action_timing, )
                .chain()
                .in_set(OnUpdate(CombatState::PlayerAttacking)),
        );
    }
}




