// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{Handle, Resource},
    sprite::TextureAtlas,
};

#[derive(Resource)]
pub struct GameAtlases {
    pub player: Handle<TextureAtlas>,
    pub tileset: Handle<TextureAtlas>,
}
