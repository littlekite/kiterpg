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
    prelude::*
};

use crate::{
    assets::GameAssets,
    plugins::{
        camera::{bundles::GameCameraBundle, components::GameCameraState},
    },
    states::AppState
};
use  crate::plugins::game::states::GameState;

#[derive(Component)]
pub struct HeaderBarUI;

#[derive(Component)]
pub struct PlayerHealthUIText;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_header_bar_ui.in_schedule(OnEnter(GameState::Running)));
    }
}

pub fn spawn_header_bar_ui(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>
) {
    let parent_node = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                margin: UiRect{
                    left: Val::Px(780.),
                    right: Val::Px(10.),
                    top: Val::Px(10.),
                    bottom: Val::Px(0.),
                },
                ..default()
            },
            //background_color: BackgroundColor(Color::WHITE),
            ..default()
        },
        HeaderBarUI,
        Name::new("Header Bar UI"),
    );
    let column_1 = (NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(33.3), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Percent(2.0)),
            ..default()
        },
        //background_color: BackgroundColor(Color::BLUE),
        ..default()
    },);

    let column_2 = (NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(66.6), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        //background_color: BackgroundColor(Color::GREEN),
        ..default()
    },);

    let player_health_background = (NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Row,
            padding: UiRect::right(Val::Percent(1.0)),
            ..default()
        },
        background_color: BackgroundColor(Color::PINK),
        ..default()
    },);

    let player_icon = (ImageBundle {
        style: Style {
            size: Size::new(Val::Auto, Val::Percent(200.0)),
            ..default()
        },
        image: UiImage::new(asset_server.load("characters/PlayerIcon.png")),
        ..default()
    },);

    let health_text = (
        TextBundle::from_section(
            "10 / 10",
            TextStyle {
                font: game_assets.font_bold.clone(),
                font_size: 36.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center),
        PlayerHealthUIText,
    );
    commands.spawn(parent_node).with_children(|commands| {
        commands.spawn(column_1).with_children(|commands| {
            commands
                .spawn(player_health_background)
                .with_children(|commands| {
                    commands.spawn(player_icon);
                    commands.spawn(health_text);
                });
        });
        commands.spawn(column_2);
    });
}