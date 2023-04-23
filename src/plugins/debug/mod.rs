// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::Plugin;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

use bevy_inspector_egui::{prelude::*, bevy_inspector, DefaultInspectorConfigPlugin};

use crate::states::AppState;
use bevy_egui::*;

use crate::plugins::{
    overworld::states::OverworldState,
    game::states::GameState,
    combat::states::CombatState
};

use bevy::prelude::*;
use egui::containers::Frame;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        #[cfg(debug_assertions)]
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(StateInspectorPlugin::<AppState>::default())
            .add_plugin(StateInspectorPlugin::<GameState>::default())
            .add_plugin(StateInspectorPlugin::<OverworldState>::default())
            .add_plugin(StateInspectorPlugin::<CombatState>::default());
    }
}

