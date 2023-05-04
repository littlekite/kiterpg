#[allow(clippy::too_many_arguments)]
use bevy::{
    prelude::{
        Commands, DespawnRecursiveExt, Entity, EventReader, Input, KeyCode, NextState, Query, Res,
        ResMut, Vec2, Without,
    },
    window::{Window, WindowFocused},
};
use crate::{states::AppState, plugins::spawner::bundles::SpawnerBundle};
use bevy::{prelude::*, render::camera};

use crate::plugins::{
    player::{
        components::{
            Player, PlayerDebuffSlowWalk, PlayerDirection, PlayerSize, PlayerVelocity,
            PlayerWalkSpeed,
        },
        states::PlayerState,
    },
    game::states::{GameState,LevelState},
    overworld::states::OverworldState,
    overworld::{CombatDescriptor,CombatStartTag},
    fade::{components::Fadeout, systems::spawn_fadeout},
    tilemap::bundles::TilemapBundle,
    atlas::resources::GameAtlases,
    combat::states::CombatState
};

use super::components::{CombatEntity,CombatFadeout, VictoryFadeout};



pub fn test_combat(
    mut player: Query<(&mut Transform,&mut PlayerState, &mut PlayerDirection), (With<Player>,Without<Camera>)>,
){

    let (
        mut player_transform,
        mut player_state,
        mut player_direct
    ) = player
        .get_single_mut().expect("0 or more than 1 `Player` found.");
    println!("{:?}",player_state);
    println!("{:?}",player_direct)
}

pub fn start_combat(
    fadeout: Query<(&Fadeout, &CombatFadeout)>,
    combat_descriptor: Query<(Entity, &Handle<CombatDescriptor>), With<CombatStartTag>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut combat_state: ResMut<NextState<CombatState>>,

) {
    
    //Check that fade completes
    if let Ok((fadeout, _)) = fadeout.get_single() {
        if fadeout.fade_in_just_finished {
            overworld_state.set(OverworldState::NotInOverworld);
            game_state.set(GameState::Combat);
            combat_state.set(CombatState::PlayerSelecting);
        }
    }
     
}

pub fn spawn_combat(
    mut commands: Commands,
    game_atlases: Res<GameAtlases>,
    combat_descriptor: Query<(Entity, &Handle<CombatDescriptor>), With<CombatStartTag>>,
    mut camera: Query<&mut Transform, (With<Camera>)>,
    combats: Res<Assets<CombatDescriptor>>,
    mut player: Query<(&mut Transform,&mut PlayerState, &mut PlayerDirection), (With<Player>,Without<Camera>)>,
    assets: Res<AssetServer>,
) {

    let mut camera = camera.single_mut();
    camera.translation = Vec3::new(112.0, 100.0, 999.0);

    //重置player的位置
    let (
        mut player_transform,
        mut player_state,
        mut player_direct
    ) = player
        .get_single_mut().expect("0 or more than 1 `Player` found.");
    *player_state = PlayerState::Fight;
    player_transform.translation = Vec3::new(154.0, 53.0, 10.);
    *player_direct = PlayerDirection::Left;
    
    commands
        .spawn(TilemapBundle::new(
            game_atlases.tileset.clone(),
            (16.0, 16.0).into(),
            (16, 20).into(),
            vec![
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4, 4, 4, 4,4,4,4, 4, 4, 4, 4, 4],
            ],
            vec![],
            None,
        ))
        .insert(Name::from("FightMap")).insert(CombatEntity);


        //add swap
   
        commands.spawn(SpawnerBundle::new(
            "Spawner A",
            0.3,
            11.0,
        )).insert(Name::from("Spawner"));
        


}


pub fn test_combat_end(
    mut commands: Commands,
    mut combat_state: ResMut<NextState<CombatState>>, 
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform,&mut PlayerState, &mut PlayerDirection), (With<Player>,Without<Camera>)>,
    fadeout: Query<&Fadeout>
) {
   
    if fadeout.iter().count() != 0 {
        return; // Only allow 1 fadeout
    }
    if input.just_pressed(KeyCode::P) {
        combat_state.set(CombatState::PlayerWins);
        let fadeout = spawn_fadeout(&mut commands);
        commands.entity(fadeout).insert(VictoryFadeout);
    }
}

pub fn transition_to_overworld(
    mut commands: Commands,
    fadeout: Query<&Fadeout, With<VictoryFadeout>>,
    mut level_state: ResMut<NextState<LevelState>>,
    mut combat_state: ResMut<NextState<CombatState>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    query: Query<Entity, With<CombatEntity>>
) {
    if let Ok(fadeout) = fadeout.get_single() {
        if fadeout.fade_in_just_finished {
            
            for entity in query.iter() {
                commands.entity(entity).despawn_recursive();
            }
             
            //appstate.set(AppState::InGame);
            //next_state.set(GameState::Running);
            level_state.set(LevelState::Overworld);
            combat_state.set(CombatState::NotInCombat);
            //overworld_state.set(OverworldState::RestoreRoom);
        } 
    }
}