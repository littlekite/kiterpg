// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::*;

use crate::states::AppState;

use self::{
    events::AnimationStartEvent,
    systems::{handle_animation_start_event, update_active_animation_clips, animate_melee},
};

use super::{game::states::GameState, combat::states::CombatState};

pub mod bundles;
pub mod components;
pub mod events;
pub mod systems;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationStartEvent>()
        /* 
        .add_systems(
            (update_active_animation_clips, handle_animation_start_event)
                .in_set(OnUpdate(AppState::InGame))
                .distributive_run_if(in_state(GameState::Running)),
        ).add_systems(
            (update_active_animation_clips, handle_animation_start_event)
                .in_set(OnUpdate(AppState::InGame))
                .distributive_run_if(in_state(GameState::Combat)),
        );
        */
        .add_systems(
            (update_active_animation_clips, handle_animation_start_event)
                .in_set(OnUpdate(AppState::InGame))
                .distributive_run_if(is_run),
        ).add_system(
            animate_melee
                .in_set(OnUpdate(CombatState::PlayerAttacking)),
        ).add_system(
            animate_melee
                .in_set(OnUpdate(CombatState::EnemyAttacking)),
        );
     
    }
}

pub fn is_run(game_state: Res<State<GameState>>) -> bool {
    if GameState::Running == game_state.0 || GameState::Combat == game_state.0{
        return true;
    }
    false
}