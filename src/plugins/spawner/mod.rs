// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{in_state, App, IntoSystemConfig, IntoSystemConfigs, OnUpdate, Plugin};

use crate::states::AppState;



use super::game::states::GameState;
pub mod bundles;
pub mod systems;
pub mod components;
pub mod states;

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        
    }
}
