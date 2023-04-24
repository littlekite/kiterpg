// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Resource,Vec3}
};

#[derive(Resource)]
pub struct CurrentRoom {
    pub current_player_translation: Vec3
}
