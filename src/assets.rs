// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // Images.
    #[asset(path = "images/player.png")]
    pub image_player: Handle<Image>,
    #[asset(path = "images/tileset.png")]
    pub image_tileset: Handle<Image>,


    // Fonts.
    #[asset(path = "fonts/qingfengfuan.ttf")]
    pub font_medium: Handle<Font>,
    #[asset(path = "fonts/qingfengfuan.ttf")]
    pub font_bold: Handle<Font>,


}
