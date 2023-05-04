// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use crate::states::AppState;
use bevy::prelude::{
    in_state, App, States,IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin, StartupSet,
};
pub mod states;
mod systems;
use self::{
    states::{GameState, LevelState}, systems::{game_setup, setup_spritesheet_maps, update_art}
};

use crate::plugins::{
    player::states::PlayerState,
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
    gameui::GameUiPlugin, fight::components::Icon
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
        .register_type::<Icon>()
        .add_plugin(AnimationPlugin)
        .add_startup_system(setup_spritesheet_maps.in_base_set(StartupSet::PreStartup))
        .add_system(update_art)
        .add_system(game_setup.in_schedule(OnEnter(AppState::InGame)));
    }
}
