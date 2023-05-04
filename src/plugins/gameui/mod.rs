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


#[derive(Component)]
pub struct Time30;



impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_header_bar_ui.in_schedule(OnEnter(GameState::Running)))
        .add_system(spawn_time_ui.in_schedule(OnEnter(GameState::Combat)))
        .add_systems(
            (update_time_ui,des_time_ui).in_set(OnUpdate(AppState::InGame))
            .distributive_run_if(in_state(GameState::Combat)),
        );
        
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct Timeleft30 {
    pub time_total: f32,
}

/// Bundle added to a fighter stub, in order to activate it.
#[derive(Bundle)]
pub struct TimeBundle {
    pub timeb: Timeleft30
}

#[derive(Component)]
pub struct TimeEntity;

pub fn update_time_ui(
    time: Res<Time>, 
    mut query: Query<(&mut Text, &mut Timeleft30)>
){
    for (mut text, mut timer) in query.iter_mut() {
        // 计时器减少时间
        timer.time_total -= time.delta_seconds();
        // 更新文本显示
        let time_left = timer.time_total.ceil() as i32;
        text.sections[0].value = format!("{}", time_left);
    }

}

fn des_time_ui(
    mut commands: Commands, time: Res<Time>, 
    mut query: Query<(&mut Text, &mut Timeleft30)>,
    query_time: Query<Entity, With<TimeEntity>>
) {
    // 遍历所有带有 TextTimer 组件的实体
    for (mut text, mut timer) in query.iter_mut() {
        timer.time_total -= time.delta_seconds();
        // 如果计时器耗尽，销毁实体
        if timer.time_total <= 0.0 {
            for entity in query_time.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}


pub fn spawn_time_ui(
    mut commands: Commands, 
    assets: Res<GameAssets>
){
    let parent_node = (
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                margin: UiRect{
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                    top: Val::Px(10.),
                    bottom: Val::Px(0.),
                },
                ..default()
            },
            //background_color: BackgroundColor(Color::WHITE),
            ..default()
        },
        Name::new("JSQUI"),
    );
    let health_text = TextBundle::from_section(
            "30",
            TextStyle {
                font: assets.font_bold.clone(),
                font_size: 150.0,
                color: Color::GOLD,
                
            },
        )
        .with_text_alignment(TextAlignment::Center);
    commands.spawn(parent_node).with_children(|commands| {
        commands.spawn(health_text).insert(TimeBundle{timeb:Timeleft30 { time_total: 30. }});
    }).insert(TimeEntity);
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