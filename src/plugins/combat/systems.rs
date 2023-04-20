#[allow(clippy::too_many_arguments)]
use bevy::{
    prelude::{
        Commands, DespawnRecursiveExt, Entity, EventReader, Input, KeyCode, NextState, Query, Res,
        ResMut, Vec2, Without,
    },
    window::{Window, WindowFocused},
};
 
use bevy::prelude::*;

use crate::plugins::{
    game::states::{GameState},
    overworld::states::OverworldState,
    overworld::{CombatDescriptor,CombatStartTag}
};

pub fn start_combat(

    combat_descriptor: Query<(Entity, &Handle<CombatDescriptor>), With<CombatStartTag>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    mut game_state: ResMut<NextState<GameState>>,

) {
    //Check that fade completes

    game_state.set(GameState::Combat);
     
}