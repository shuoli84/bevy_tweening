use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_tweening::{lens::*, *};
use std::time::Duration;

const NORMAL_COLOR: Color = Color::rgba(162. / 255., 226. / 255., 95. / 255., 1.);
const HOVER_COLOR: Color = Color::AZURE;
const CLICK_COLOR: Color = Color::ALICE_BLUE;
const TEXT_COLOR: Color = Color::rgba(83. / 255., 163. / 255., 130. / 255., 1.);

fn main() {
    App::default()
        .insert_resource(WindowDescriptor {
            title: "Menu".to_string(),
            width: 800.,
            height: 400.,
            present_mode: bevy::window::PresentMode::Fifo, // vsync
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .add_system(interaction)
        .add_plugin(TweeningPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let font = asset_server.load("fonts/FiraMono-Regular.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::all(Val::Px(0.)),
                margin: UiRect::all(Val::Px(16.)),
                padding: UiRect::all(Val::Px(16.)),
                flex_direction: FlexDirection::ColumnReverse,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: UiColor(Color::NONE),
            ..default()
        })
        .insert(Name::new("menu"))
        .with_children(|container| {
            for (text, label) in [
                ("Continue", ButtonLabel::Continue),
                ("New Game", ButtonLabel::NewGame),
                ("Settings", ButtonLabel::Settings),
                ("Quit", ButtonLabel::Quit),
            ] {
                let tween_scale = Tween::new(
                    EaseFunction::BounceOut,
                    Duration::from_secs(2),
                    TransformScaleLens {
                        start: Vec3::splat(0.01),
                        end: Vec3::ONE,
                    },
                );
                let animator = Animator::new(tween_scale);
                container
                    .spawn_bundle(ButtonBundle {
                        node: Node {
                            size: Vec2::new(300., 80.),
                        },
                        style: Style {
                            min_size: Size::new(Val::Px(300.), Val::Px(80.)),
                            margin: UiRect::all(Val::Px(8.)),
                            padding: UiRect::all(Val::Px(8.)),
                            align_content: AlignContent::Center,
                            align_items: AlignItems::Center,
                            align_self: AlignSelf::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        color: UiColor(NORMAL_COLOR),
                        transform: Transform::from_scale(Vec3::splat(0.01)),
                        ..default()
                    })
                    .insert(Name::new(format!("button:{}", text)))
                    .insert(animator)
                    .insert(label)
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::from_section(
                                text.to_string(),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 48.0,
                                    color: TEXT_COLOR,
                                },
                            )
                            .with_alignment(TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            }),
                            ..default()
                        });
                    });
            }
        });
}

#[derive(Component, Clone, Copy)]
enum ButtonLabel {
    Continue,
    NewGame,
    Settings,
    Quit,
}

fn interaction(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Transform, &Interaction, &mut UiColor, &ButtonLabel),
        Changed<Interaction>,
    >,
) {
    for (entity, transform, interaction, mut color, button_label) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = CLICK_COLOR.into();

                match button_label {
                    ButtonLabel::Continue => {
                        println!("Continue clicked");
                    }
                    ButtonLabel::NewGame => {
                        println!("NewGame clicked");
                    }
                    ButtonLabel::Settings => {
                        println!("Settings clicked");
                    }
                    ButtonLabel::Quit => {
                        println!("Quit clicked");
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVER_COLOR.into();
                commands.entity(entity).insert(Animator::new(Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_secs_f32(0.2),
                    TransformScaleLens {
                        start: Vec3::ONE,
                        end: Vec3::splat(1.1),
                    },
                )));
            }
            Interaction::None => {
                *color = NORMAL_COLOR.into();
                let start_scale = transform.scale;

                commands.entity(entity).insert(Animator::new(Tween::new(
                    EaseFunction::QuadraticIn,
                    Duration::from_secs_f32(0.2),
                    TransformScaleLens {
                        start: start_scale,
                        end: Vec3::ONE,
                    },
                )));
            }
        }
    }
}
