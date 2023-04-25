// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use crate::states::AppState;
use bevy::prelude::{
    in_state, App, States,IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin,
};
pub mod states;
mod systems;
use self::{
    states::{GameState, LevelState}, systems::game_setup
};

use crate::plugins::{
    fade::components::Fadeout
};
use super::{
    animation::AnimationPlugin,
    debug::DebugPlugin,
    camera::CameraPlugin,
    tilemap::TilemapPlugin,
    overworld::OverworldPlugin,
    player::PlayerPlugin,
    combat::CombatPlugin,
    fade::FadeInPlugin,
    gameui::GameUiPlugin
    //enemy::EnemyPlugin,
};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
        .add_state::<LevelState>()
        .add_plugin(DebugPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(OverworldPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(GameUiPlugin)
        //.add_plugin(EnemyPlugin)
        .add_plugin(FadeInPlugin)
        .register_type::<Fadeout>()
        .add_plugin(AnimationPlugin)
        .add_system(game_setup.in_schedule(OnEnter(AppState::InGame)));
    }
}
