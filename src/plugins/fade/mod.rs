
use bevy::prelude::*;
use bevy::prelude::{in_state, IntoSystemConfig, OnUpdate, Plugin};

pub mod states;
pub mod systems;
pub mod components;
pub struct FadeInPlugin;

use self::{
    systems::{update_fadeout}
};

impl Plugin for FadeInPlugin {
    fn build(&self, app: &mut App) {
        //PreUpdate prevents finishing at the end of one update and clearing the flag at the beginning of the next, causing a missed event
        app.add_system(update_fadeout.in_base_set(CoreSet::PreUpdate));
    }
}