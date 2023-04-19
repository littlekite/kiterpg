// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use crate::states::AppState;
use bevy::prelude::{
    in_state, App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin,
};
pub mod states;
mod systems;
use self::{
    states::{GameState, LevelState}, systems::game_setup
};

use super::{
    debug::DebugPlugin,
    tilemap::TilemapPlugin,
    overworld::OverworldPlugin
};


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
        .add_state::<LevelState>()
        .add_plugin(DebugPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(OverworldPlugin)
        .add_system(game_setup.in_schedule(OnEnter(AppState::InGame)));
    }
}
