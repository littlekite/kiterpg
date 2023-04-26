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
    overworld::states::OverworldState,
    combat::states::CombatState,
    fade::systems::spawn_fadeout,
    overworld::resources::CurrentRoom,
    combat::components::CombatFadeout
};

use crate::states::AppState;



pub fn player_position(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut room: ResMut<CurrentRoom>,
    mut query: Query<&Transform, With<Player>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    mut appstate: ResMut<NextState<AppState>>,
    mut combat_state: ResMut<NextState<CombatState>>
) {
        if query.is_empty() {
            return;
        }
        let player =  query.single();
        room.current_player_translation = player.translation;
        //println!("{:?}",room.current_player_translation.x);
        //println!("{:?}",room.current_player_translation.y);
}