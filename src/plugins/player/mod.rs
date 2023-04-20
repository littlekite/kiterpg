// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{
    in_state, IntoSystemAppConfig, IntoSystemConfig, IntoSystemConfigs, OnEnter, OnUpdate, Plugin,
};

use crate::states::AppState;

use self::systems::{
    animation::player_animation,
    movement::{player_movement, player_movement_input, player_movement_reset},
    fight::{player_fight}
};

use super::game::states::GameState;

pub mod bundles;
pub mod components;
pub mod states;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(player_movement_reset.in_schedule(OnEnter(GameState::Running)))
            .add_systems(
                (
                    player_movement_input.before(player_movement),
                    player_movement,
                    player_animation,
                    player_fight
                )
                    .in_set(OnUpdate(AppState::InGame))
                    .distributive_run_if(in_state(GameState::Running)),
            );
    }
}
