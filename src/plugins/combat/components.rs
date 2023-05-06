use bevy::prelude::{Component, Entity};
use bevy::{prelude::States, reflect::Reflect};
use serde::{Serialize, Deserialize};

#[derive(Component)]
pub struct VictoryFadeout;

#[derive(Component)]
pub struct CombatEntity;

#[derive(Component)]
pub struct CombatFadeout;

#[derive(Component, Reflect)]
pub struct PlayerAttack {
    pub target: Entity,
    pub slot: usize,
}
