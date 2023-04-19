// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{
    in_state, App, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnExit, OnUpdate, Plugin,
};

use self::systems::{
    overworld_setup,
};

use super::game::states::{LevelState, StoryState};

pub mod systems;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(overworld_setup.in_schedule(OnEnter(LevelState::Overworld)));
    }
}
