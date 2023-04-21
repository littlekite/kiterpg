

use bevy::prelude::*;

use bevy::{prelude::States, reflect::Reflect};

use super::{
    states::{FadeoutState},
};

#[derive(Component, Reflect)]
pub struct Fadeout {
    pub fade_in_just_finished: bool,
    pub in_timer: Timer,
    pub hold_timer: Timer,
    pub out_timer: Timer,
    pub state: FadeoutState,
}