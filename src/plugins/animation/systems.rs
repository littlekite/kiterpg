use crate::plugins::{fight::{Weapon, AttackAnimation, Attack, WeaponAttackType, AttackStage}, player::components::Player};

// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use super::{
    components::{ActiveAnimationClipName, AnimationClips, AnimationTimer},
    events::AnimationStartEvent,
};
use bevy::{
    prelude::{Entity, EventReader, Query, Res, Without, Children, Transform, With, Quat},
    sprite::TextureAtlasSprite,
    time::Time,
};
use bevy_easings::Lerp;
use std::time::Duration;

pub fn update_active_animation_clips(
    time: Res<Time>,
    mut query: Query<(
        &AnimationClips,
        &ActiveAnimationClipName,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (all_clips, active_clip_name, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            // Update the texture-index of the sprite each time the `AnimationTimer` finishes.
            if let Some(active_clip) = all_clips.0.get_key_value(&active_clip_name.0) {
                sprite.index = if sprite.index >= active_clip.1.index_last {
                    active_clip.1.index_first
                } else {
                    sprite.index + 1
                };
            }
        }
    }
}

pub fn handle_animation_start_event(
    mut query: Query<(
        Entity,
        &AnimationClips,
        &mut ActiveAnimationClipName,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
    mut start_event_reader: EventReader<AnimationStartEvent>,
) {
    for start_event in start_event_reader.iter() {
        for (entity, all_clips, mut active_clip_name, mut animation_timer, mut sprite) in &mut query
        {
            if start_event.target_entity != entity {
                continue;
            }

            // Find the new active clip with the clip-name provided by the event and activate it.
            if let Some(new_active_clip) = all_clips.0.get_key_value(&start_event.clip_name) {
                active_clip_name.0 = start_event.clip_name.clone();
                sprite.index = new_active_clip.1.index_first;

                animation_timer
                    .0
                    .set_duration(Duration::from_secs_f32(new_active_clip.1.playback_speed));

                if start_event.reset_timer {
                    animation_timer.0.reset();
                }
            };
        }
    }
}


pub fn animate_melee(
    mut attacker: Query<(&mut Transform), (With<Player>,Without<Weapon>)>,
    attack: Query<(&Attack, &AttackAnimation)>,
    mut weapon: Query<&mut Transform, With<Weapon>>,
){
    if let Ok((attack, animation)) = attack.get_single() {
        if !matches!(attack.attack_type, WeaponAttackType::Melee) {
            return;
        }
        let (mut transform) = attacker
        .get_mut(attack.attacker)
        .expect("Attacker has no weapon");
        match attack.stages[attack.current_stage].0 {
            AttackStage::WalkUp => {
                transform.translation.x =
                    (animation.starting_x).lerp(&animation.ending_x, &attack.timer.percent());
                transform.translation.y =
                    (animation.starting_y).lerp(&animation.ending_y, &attack.timer.percent());
               
            }
            AttackStage::Action => {
                transform.translation.x = animation.ending_x;
                transform.translation.y = animation.ending_y;
                if attack.timer.percent() < 0.5 {
                    
                } else {
                    
                }
            }
            AttackStage::CoolDown => {
                transform.translation.x =
                    (animation.ending_x).lerp(&animation.starting_x, &attack.timer.percent());
                transform.translation.y =
                    (animation.ending_y).lerp(&animation.starting_y, &attack.timer.percent());
                
            }
            _ => {}
        }

    }
}
