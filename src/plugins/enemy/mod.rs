use  bevy::prelude::*;

pub mod states;
pub mod bundles;
pub mod components;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
    }
}