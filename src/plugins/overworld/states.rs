use bevy::{prelude::States, reflect::Reflect};

#[derive(States, PartialEq, Eq, Debug, Default, Clone, Hash, Reflect)]
pub enum OverworldState {
    #[default]
    LoadingRoom,
    RestoreRoom,
    CombatStarting,
    NotInOverworld
}