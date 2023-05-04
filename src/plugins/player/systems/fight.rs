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
    combat::components::CombatFadeout, enemy::components::Enemy
};

use crate::states::AppState;



pub fn player_fight(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut PlayerState, &mut PlayerDirection), With<Player>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    mut appstate: ResMut<NextState<AppState>>,
    mut enemy:Query<(Entity, &mut Transform), With<Enemy>>,
    mut combat_state: ResMut<NextState<CombatState>>
) {
        if query.is_empty() {
            return;
        }
    
        let (mut player_state, mut player_direction) = query
            .get_single_mut()
            .expect("0 or more than 1 `Player` found.");
    
    
        if keyboard.just_pressed(KeyCode::L) {
            //状态更改
            overworld_state.set(OverworldState::CombatStarting);
            
            let fadeout = spawn_fadeout(&mut commands);
            commands.entity(fadeout).insert(CombatFadeout);
            
            //appstate.set(AppState::Combat);
        }


        if keyboard.just_pressed(KeyCode::A) {
            //随机选取enemy进行物理攻击
            println!("A is pressed!");
            /*
            for (ent,tran) in &mut enemy{
                    println!("{:?}",ent.index())
            }
            */     
        }

       
    
}