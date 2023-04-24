// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{
    in_state, App, States,  IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnExit, OnUpdate, Plugin,
    
};
use bevy::{prelude::*,reflect::TypeUuid};
use serde::{Deserialize, Serialize};

use self::systems::{
    overworld_setup,
};

use super::game::states::{LevelState, StoryState};

pub mod systems;
pub mod states;

use self::{
    states::{OverworldState}
};

pub use bevy_common_assets::ron::RonAssetPlugin;

pub struct OverworldPlugin;
pub mod resources;


impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<OverworldState>()
        .add_system(overworld_setup.in_schedule(OnEnter(LevelState::Overworld)));
    }
}

#[derive(Component, Deserialize, TypeUuid)]
#[uuid = "ab5d5e61-fbb6-403f-a19f-39dbe413a440"]
pub struct RoomDescriptor {
    
}



#[derive(Component)]
pub struct CombatStartTag;

#[derive(Component, Serialize, Deserialize, TypeUuid)]
#[uuid = "9d0b9466-8797-486e-a930-bcb696f8e2f3"]
pub struct CombatDescriptor {
   
}
