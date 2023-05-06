
use bevy::{prelude::{States, Component}, reflect::Reflect};
use serde::{Serialize, Deserialize};

#[derive(States, PartialEq, Eq, Debug, Default, Clone, Hash, Reflect)]
pub enum CombatState {
    #[default]
    NotInCombat,
    PlayerSelecting,
    PlayerAttacking,
    EnemyAttacking,
    EnemyDying,
    PlayerWins,
}

