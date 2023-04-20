use bevy::{
    prelude::{Input, KeyCode, Query, Rect, Res, Transform, Vec2, With, Without,NextState},
    time::Time,
    prelude::*
};

use crate::plugins::{
    player::{
        components::{
            Player, PlayerDebuffSlowWalk, PlayerDirection, PlayerSize, PlayerVelocity,
            PlayerWalkSpeed,
        },
        states::PlayerState,
    },
    tilemap::components::TilemapColliders,
    overworld::states::OverworldState
};

pub fn player_fight(
        keyboard: Res<Input<KeyCode>>,
        mut query: Query<(&mut PlayerState, &mut PlayerDirection), With<Player>>,
        mut overworld_state: ResMut<NextState<OverworldState>>,
    ) {
        if query.is_empty() {
            return;
        }
    
        let (mut player_state, mut player_direction) = query
            .get_single_mut()
            .expect("0 or more than 1 `Player` found.");
    
    
        if keyboard.pressed(KeyCode::L) {
            //状态更改
            overworld_state.set(OverworldState::CombatStarting);
        }
    
}