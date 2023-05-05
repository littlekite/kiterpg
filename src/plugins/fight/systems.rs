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

use super::{components::Icon, IconBundle, CurrentSelectedMenuItem, SelectionIcon, WeaponBundle, Weapon, WeaponIcon};




pub fn textfun(
    keyboard: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform,&mut PlayerState, &mut PlayerDirection), (With<Player>,Without<Camera>)>,
    mut next_state: ResMut<NextState<CombatState>>,
){

    let (
        mut player_transform,
        mut player_state,
        mut player_direct
    ) = player
        .get_single_mut().expect("0 or more than 1 `Player` found.");
    //println!("{:?}",player_state);
    //println!("{:?}",player_direct);

    if keyboard.just_pressed(KeyCode::Space) {
        //随机选取enemy进行物理攻击
        println!("Attack Selected!");
        /*
        for (ent,tran) in &mut enemy{
                println!("{:?}",ent.index())
        }
        */
        next_state.set(CombatState::PlayerAttacking);
        
    }

}

pub fn spawn_player_attack_icons(mut commands: Commands) {
    
    commands.spawn((
        WeaponBundle::new(Vec2::new(188.0, 70.0), Weapon::BasicSpear, Vec2::splat(0.75)),
        WeaponIcon(0),
        Name::new("SpearIcon"),
    ));

    commands.spawn((
        WeaponBundle::new(
            Vec2::new(205.0, 70.0),
            Weapon::BasicStaffOrange,
            Vec2::splat(0.75),
        ),
        WeaponIcon(1),
        Name::new("StaffIcon"),
    ));


    commands.spawn((
        IconBundle::new(Vec2::new(188.0, 60.0), Icon::Pointer, Vec2::splat(0.5)),
        CurrentSelectedMenuItem {
            selection: 0,
            slots: 2,
        },
        SelectionIcon,
        Name::new("SelectionIcon"),
    ));
}


pub fn player_select_attack(
    mut selection: Query<&mut CurrentSelectedMenuItem, With<SelectionIcon>>,
    keyboard: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<CombatState>>,
) {
    for mut selection in &mut selection {
        if keyboard.just_pressed(KeyCode::A) {
            selection.selection -= 1;
        }
        if keyboard.just_pressed(KeyCode::D) {
            selection.selection += 1;
        }
        if keyboard.just_pressed(KeyCode::Space) {
            info!("Attack Selected");
            next_state.set(CombatState::PlayerAttacking);
        }
    }
}

pub fn update_icon_location(
    mut selection: Query<(&mut Transform, &CurrentSelectedMenuItem), With<SelectionIcon>>,
) {
    for (mut transform, selection) in &mut selection {
        let location = (selection.selection.rem_euclid(selection.slots)) as f32;
        transform.translation = Vec3::new(188.0 + location*18., 60.0, 850.);
    }
}