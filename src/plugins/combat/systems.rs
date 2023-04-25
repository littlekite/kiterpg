#[allow(clippy::too_many_arguments)]
use bevy::{
    prelude::{
        Commands, DespawnRecursiveExt, Entity, EventReader, Input, KeyCode, NextState, Query, Res,
        ResMut, Vec2, Without,
    },
    window::{Window, WindowFocused},
};
use crate::states::AppState;
use bevy::prelude::*;

use crate::plugins::{
    game::states::{GameState,LevelState},
    overworld::states::OverworldState,
    overworld::{CombatDescriptor,CombatStartTag},
    fade::{components::Fadeout, systems::spawn_fadeout},
    tilemap::bundles::TilemapBundle,
    atlas::resources::GameAtlases,
    combat::states::CombatState
};

use super::components::{CombatEntity,CombatFadeout, VictoryFadeout};


pub fn start_combat(
    fadeout: Query<(&Fadeout, &CombatFadeout)>,
    combat_descriptor: Query<(Entity, &Handle<CombatDescriptor>), With<CombatStartTag>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    mut game_state: ResMut<NextState<GameState>>,

) {
    //Check that fade completes
    if let Ok((fadeout, _)) = fadeout.get_single() {
        if fadeout.fade_in_just_finished {
            overworld_state.set(OverworldState::NotInOverworld);
            game_state.set(GameState::Combat);
        }
    }
     
}

pub fn spawn_combat(
    mut commands: Commands,
    game_atlases: Res<GameAtlases>,
    combat_descriptor: Query<(Entity, &Handle<CombatDescriptor>), With<CombatStartTag>>,
    mut camera: Query<&mut Transform, (With<Camera>)>,
    combats: Res<Assets<CombatDescriptor>>,
    assets: Res<AssetServer>,
) {

    let mut camera = camera.single_mut();
    camera.translation = Vec3::new(112.0, 100.0, 100.0);
    
    commands
        .spawn(TilemapBundle::new(
            game_atlases.tileset.clone(),
            (16.0, 16.0).into(),
            (16, 20).into(),
            vec![
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4,4,4,4,4,4,4, 4, 4, 4, 4, 4],
            ],
            vec![],
            None,
        ))
        .insert(Name::from("fightmap")).insert(CombatEntity);


}


pub fn test_combat_end(
    mut commands: Commands,
    mut combat_state: ResMut<NextState<CombatState>>, 
    input: Res<Input<KeyCode>>,
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