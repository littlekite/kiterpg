
use serde::{Deserialize, Serialize};

use bevy::prelude::*;
use bevy::prelude::{in_state, IntoSystemConfig, OnUpdate, Plugin};
pub use bevy_common_assets::ron::RonAssetPlugin;


use crate::plugins::{
    overworld::states::OverworldState,
    overworld::{CombatDescriptor,CombatStartTag},
    game::states::GameState
};
use crate::states::AppState;

mod systems;
pub mod components;
pub mod states;

use self::{
    states::CombatState,
    systems::{start_combat,spawn_combat,test_combat_end,transition_to_overworld}
};

use super::tilemap::systems::colliders::spawn_colliders;
use super::tilemap::systems::tiles::spawn_tiles;


pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CombatState>()
            .add_plugin(RonAssetPlugin::<CombatDescriptor>::new(&["combat.ron"]))
            .add_system(start_combat.in_set(OnUpdate(OverworldState::CombatStarting)))
            .add_system(test_combat_end)
            .add_system(transition_to_overworld)
            .add_system(spawn_tiles)
            .add_system(spawn_combat.in_schedule(OnEnter(GameState::Combat)));
    }
}

