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

use super::components::{CombatEntity};

pub fn start_combat(

    combat_descriptor: Query<(Entity, &Handle<CombatDescriptor>), With<CombatStartTag>>,
    mut overworld_state: ResMut<NextState<OverworldState>>,
    mut game_state: ResMut<NextState<GameState>>,

) {
    //Check that fade completes
    overworld_state.set(OverworldState::NotInOverworld);
    game_state.set(GameState::Combat);
     
}

pub fn spawn_combat(
    mut commands: Commands,
    combat_descriptor: Query<(Entity, &Handle<CombatDescriptor>), With<CombatStartTag>>,
    mut camera: Query<&mut Transform, (With<Camera>)>,
    combats: Res<Assets<CombatDescriptor>>,
    assets: Res<AssetServer>,
) {

    let mut camera = camera.single_mut();
    camera.translation = Vec3::new(0.0, 0.0, 999.0);
    
    /* 
    let (entity, combat_desc) = &combat_descriptor.single();
    commands.entity(*entity).despawn_recursive();
    // FIXME this is a kinda unsound assumption...
    let combat_desc = combats
        .get(combat_desc)
        .expect("Combat should have loaded by end of fade...");
    */
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(26.0, 11.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 100. - 0.1),
            texture: assets.load("CaveBackground.png"),
            ..default()
        },
        Name::new("Background"),
        CombatEntity,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(11.0, 7.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -1.2, 100.),
            texture: assets.load("Stage.png"),
            ..default()
        },
        Name::new("Background"),
        CombatEntity,
    ));
}