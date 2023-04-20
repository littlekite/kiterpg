
use serde::{Deserialize, Serialize};

use bevy::prelude::*;
use bevy::prelude::{in_state, IntoSystemConfig, OnUpdate, Plugin};
pub use bevy_common_assets::ron::RonAssetPlugin;

mod states;
use crate::plugins::{
    overworld::states::OverworldState,
    overworld::{CombatDescriptor,CombatStartTag}
};

mod systems;
use self::{
    states::CombatState,
    systems::start_combat
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RonAssetPlugin::<CombatDescriptor>::new(&["combat.ron"]))
            .add_system(start_combat.in_set(OnUpdate(OverworldState::CombatStarting)));
    }
}

