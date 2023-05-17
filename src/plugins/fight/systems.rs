#[allow(clippy::too_many_arguments)]
use bevy::{
    prelude::{
        Commands, DespawnRecursiveExt, Entity, EventReader, Input, KeyCode, NextState, Query, Res,
        ResMut, Vec2, Without,
    },
    window::{Window, WindowFocused},
};
use crate::{states::AppState, plugins::{spawner::bundles::SpawnerBundle, enemy::{components::{Enemy, EnemyHealth, EnemyDamage, EnemyIsDead, Enemyslot}, states::EnemyState}, combat::{states::CombatState, components::{CombatEntity, PlayerAttack}}}};
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
    atlas::resources::GameAtlases
};

use super::{components::Icon, IconBundle, CurrentSelectedMenuItem, SelectionIcon, WeaponBundle, Weapon, WeaponIcon, WeaponAttackType, AttackBundle, AttackAnimation, Attack, AttackStage, ActionTiming, Action};




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


pub fn attack_flow(
    mut attack: Query<&mut Attack>,
    time: Res<Time>,
    state: Res<State<CombatState>>,
    mut next_state: ResMut<NextState<CombatState>>,
    mut player_state:ResMut<NextState<PlayerState>>,
    player: Query<(Entity), (With<Player>,Without<Enemy>)>,
    enemy: Query<(Entity), (With<Enemy>, Without<Player>)>,    
){

    for mut attack in &mut attack {
       
        attack.timer.tick(time.delta());
        if attack.timer.just_finished() {

            let finished_stage = &attack.stages[attack.current_stage].0;
     
            if matches!(finished_stage, AttackStage::Action) {
                
            }
               //Turn ending
            attack.current_stage += 1;
            if attack.current_stage >= attack.stages.len() {
                attack.current_stage = attack.stages.len() - 1;
                match state.0 {
                    CombatState::PlayerAttacking => 
                    {
                        next_state.set(CombatState::EnemyAttacking);
                        println!("do enmey deamg animation")
                    },
                    CombatState::EnemyAttacking => 
                    {
                        next_state.set(CombatState::PlayerSelecting);
                        //将player状态设置为被击中
                        player_state.set(PlayerState::OnFight);    
                        println!("do player deamg animation")
                        
                    },
                    _ => unreachable!("Can't finish attack in this state"),
                }
                return;
            }

            let next_timer = attack.stages[attack.current_stage].1;
            attack.timer = Timer::from_seconds(next_timer, TimerMode::Once);
        }
    }
}


impl Weapon {
    pub fn attack_type(&self) -> WeaponAttackType {
        match self {
            Weapon::BasicSpear => WeaponAttackType::Melee,
            Weapon::BasicStaffOrange => WeaponAttackType::Range,
        }
    }

    pub fn get_attack_bundle(
        &self,
        player: bool,
        attacker: Entity,
        target: Entity,
        slot: usize,
    ) -> AttackBundle {
        let animation = if player {
            match self {
                Weapon::BasicStaffOrange => AttackAnimation {
                    starting_x: 154.,
                    starting_y: 53.,
                    ending_x: 60.0 + 20.0 * slot as f32,
                    ending_y: 110.0 + 20.0 * slot as f32,
                    max_weapon_rotation: -1.0,
                },
                Weapon::BasicSpear => AttackAnimation {
                    starting_x: 154.0,
                    starting_y: 53.,
                    ending_x: 60.0 + 20.0 * slot as f32,
                    ending_y: 110.0 + 20.0 * slot as f32,
                    max_weapon_rotation: -1.0,
                },
            }
        } else {
            AttackAnimation {
                starting_x: 112.,
                starting_y: 100.,
                ending_x: 154.0,
                ending_y: 53.,
                max_weapon_rotation: 6.0,
            }
        };
        let attack = match self {
            Weapon::BasicStaffOrange => Attack {
                attacker,
                target,
                current_stage: 0,
                stages: vec![
                    (AttackStage::Charge, 0.2),
                    (AttackStage::WalkUp, 0.7),
                    (AttackStage::Action, 0.2),
                    (AttackStage::CoolDown, 0.7),
                ],
                action: Action {
                    stage: AttackStage::Action,
                    action_input: ActionTiming::NotEntered,
                },
                attack_type: self.attack_type(),
                timer: Timer::from_seconds(0.2, TimerMode::Once),
            },
            Weapon::BasicSpear => Attack {
                attacker,
                target,
                current_stage: 0,
                stages: vec![
                    (AttackStage::WalkUp, 0.7),
                    (AttackStage::Action, 0.2),
                    (AttackStage::CoolDown, 0.7),
                ],
                action: Action {
                    stage: AttackStage::Action,
                    action_input: ActionTiming::NotEntered,
                },
                attack_type: self.attack_type(),
                timer: Timer::from_seconds(0.7, TimerMode::Once),
            },
        };

        AttackBundle { attack, animation }
    }
}

pub fn player_action_timing(
    mut attack: Query<&mut Attack>, 
    keyboard: Res<Input<KeyCode>>
) {
    for mut attack in &mut attack {
        if keyboard.just_pressed(KeyCode::Space)
            && attack.action.action_input == ActionTiming::NotEntered
        {
            println!("{:?}",attack.current_stage);
            match attack.stages[attack.current_stage].0 {
                //FIXME should look at what is the current stage regardless of fixed step flow
                AttackStage::WalkUp => {
                    if attack.timer.percent() > 0.7 {
                        attack.action.action_input = ActionTiming::Early;
                        println!("s")
                    }
                }
                AttackStage::Action => {
                    attack.action.action_input = ActionTiming::Critical;
                    println!("ssd")
                }
                AttackStage::CoolDown => {
                    println!("1s");
                    if attack.timer.percent() < 0.3 {
                        attack.action.action_input = ActionTiming::Late;
                        println!("sd")
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn spawn_enemy_attack(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    //Without dying enemies
    enemy: Query<(Entity, &Transform, &Enemy, &Enemyslot), (Without<Player>)>,
) {
    let (enemy, transform, _, slo) = enemy
        .iter()
        .min_by_key(|(_, _, enemy,enemy_slo)| enemy_slo.0)
        .expect("No enemy to attack");

    let player = player.get_single().expect("One player only!");
    let mut attack = Weapon::BasicSpear.get_attack_bundle(false, enemy, player, 0);
    attack.animation.starting_x = transform.translation.x;
    commands.spawn(attack);
}

pub fn spawn_player_attack(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    locked_attack: Query<(Entity, &Weapon, &PlayerAttack)>,
){
    let (entity, weapon, attack) = locked_attack.get_single().expect("No attack!");
    //This might all need to be reworked, maybe the weapon creates it's whole attack comp...
    let player = player.get_single().expect("One player only!");

    commands.entity(entity).insert(weapon.get_attack_bundle(
        true,
        player,
        attack.target,
        attack.slot,
    ));
}

pub fn lock_in_attack(
    mut commands: Commands,
    selection: Query<&CurrentSelectedMenuItem, With<SelectionIcon>>,
    mut enemy_query: Query<
    (Entity, &Transform,&Enemyslot),
    (With<Enemy>, Without<Player>),
    >,  
    weapon_icons: Query<(&WeaponIcon, &Weapon)>,
) {
    
    let (entity, enemy,slo) = enemy_query
    .iter()
    .min_by_key(|(_, _,enemy_slo)| enemy_slo.0)
    .expect("No enemy to target");
     info!("Locking in attack");

    let selection = selection.single();
    let slot = selection.selection.rem_euclid(selection.slots);

    for (icon, weapon) in &weapon_icons {
        if icon.0 == slot {
            info!("Attacked locked");
            commands.spawn((
                weapon.clone(),
                PlayerAttack {
                    target: entity,
                    slot: 0,
                },
                CombatEntity,
            ));
            return;
        }
    }

    unreachable!("Player didn't select anything :(");
}