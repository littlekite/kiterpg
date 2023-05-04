

use bevy::prelude::*;

use bevy::{prelude::States, reflect::Reflect};



#[derive(Component, Clone, PartialEq, Eq, Hash, Default, Reflect)]
pub enum Icon {
    #[default]
    Pointer,
    KeyE,
}