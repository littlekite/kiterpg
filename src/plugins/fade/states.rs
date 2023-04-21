

use bevy::{prelude::*};
use bevy::{prelude::States, reflect::Reflect};



#[derive(Reflect)]
pub enum FadeoutState {
    FadingIn,
    Hold,
    FadingOut,
}