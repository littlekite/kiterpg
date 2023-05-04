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

use super::{components::Icon, IconBundle, CurrentSelectedMenuItem, SelectionIcon};




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
        IconBundle::new(Vec2::new(188.0, 60.0), Icon::Pointer, Vec2::splat(0.5)),
        CurrentSelectedMenuItem {
            selection: 0,
            slots: 2,
        },
        SelectionIcon,
        Name::new("SelectionIcon"),
    ));
}