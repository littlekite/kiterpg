// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::{in_state, IntoSystemConfig, OnUpdate, Plugin};

use crate::states::AppState;


pub mod bundles;
pub mod components;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
      
    }
}
