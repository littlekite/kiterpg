// Copyright (c) 2023 Paul
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use bevy::{
    prelude::{
        default, App, Color, Commands, DespawnRecursiveExt, Entity, Input, IntoSystemAppConfig,
        IntoSystemConfig, KeyCode, NextState, OnEnter, OnExit, OnUpdate, Plugin, Query, Res,
        ResMut, TextBundle, Vec2, Without,
    },
    text::{TextAlignment, TextSection, TextStyle},
    ui::{AlignSelf, PositionType, Size, Style, UiRect},
    window::Window,
};

use crate::{
    assets::GameAssets,
    plugins::{
        camera::{bundles::GameCameraBundle, components::GameCameraState},
    },
    states::AppState,
};


pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            intro_setup.in_schedule(OnEnter(AppState::Introduction)),
            intro_start_game.in_set(OnUpdate(AppState::Introduction)),
            intro_cleanup.in_schedule(OnExit(AppState::Introduction)),
        ));
    }
}

pub fn intro_setup(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    commands.spawn(GameCameraBundle::new(0.2, GameCameraState::Static));
    commands.spawn((TextBundle::from_sections([
        TextSection::new(
            "丛林传说",
            TextStyle {
                font: game_assets.font_bold.clone(),
                font_size: 150.0,
                color: Color::WHITE,
            },
        ),
        TextSection::new(
            "\n这是一次新的冒险故事....\n",
            TextStyle {
                font: game_assets.font_bold.clone(),
                font_size: 30.0,
                color: Color::WHITE,
            },
        ),
        TextSection::new(
            "\n按[Space]开始",
            TextStyle {
                font: game_assets.font_bold.clone(),
                font_size: 60.0,
                color: Color::WHITE,
            },
        ),
    ])
    .with_text_alignment(TextAlignment::Center)
    .with_style(Style {
        size: Size {
            width: bevy::ui::Val::Percent(100.0),
            ..default()
        },
        position: UiRect::left(bevy::ui::Val::Px(350.0)),
        position_type: PositionType::Relative,
        align_self: AlignSelf::Center,
        ..default()
    }),));
}

pub fn intro_start_game(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}

pub fn intro_cleanup(mut commands: Commands, query: Query<Entity, Without<Window>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
