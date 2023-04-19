use bevy::{
    prelude::*,
    render::texture::ImageSampler,
    window::WindowResolution,
    DefaultPlugins,
};

use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

mod states;
use states::AppState;

mod assets;
use assets::GameAssets;

mod plugins;
use plugins::{
    game::GamePlugin,
    atlas::AtlasPlugin,
    intro::IntroPlugin,
};


fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin {
                // 像素画放大后仍保证清晰
                default_sampler: ImageSampler::nearest_descriptor(),
            })
            .set(WindowPlugin {
                //设置窗口大小 1100*750
                primary_window: Some(Window {
                    #[cfg(target_os = "windows")]
                    position: WindowPosition::Centered(MonitorSelection::Primary), //窗口居中
                    resolution: WindowResolution::new(1200.0, 800.0),
                    ..default()
                }),
                ..default()
            }),
    )
    .insert_resource(ClearColor(Color::GRAY))
    .add_state::<AppState>()
    .add_loading_state(
        LoadingState::new(AppState::LoadingAssets).continue_to_state(AppState::BuildingAtlases),
    )
    .add_collection_to_loading_state::<_, GameAssets>(AppState::LoadingAssets)
    .add_plugin(GamePlugin)
    .add_plugin(AtlasPlugin)
    .add_plugin(IntroPlugin);

    app.run();
}

