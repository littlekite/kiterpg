
use bevy::{prelude::States, reflect::Reflect};

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