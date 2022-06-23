use crate::loading::{FontAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_score))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_score));
    }
}

#[derive(Default, Inspectable)]
pub struct Score(pub f32);

#[derive(Component)]
struct ScoreText;
#[derive(Component)]
struct CurrentEggText;
#[derive(Component)]
struct MaxEggText;

fn spawn_score(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Percent(100.)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
                position: Rect {
                    right: Val::Px(0.),
                    top: Val::Px(0.),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: UiColor(Color::Rgba {
                red: 0.7,
                green: 0.7,
                blue: 0.7,
                alpha: 0.7,
            }),
            ..Default::default()
        })
        // Money
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        position: Rect {
                            left: Val::Px(5.),
                            top: Val::Px(5.),
                            ..default()
                        },
                        ..default()
                    },
                    color: UiColor(Color::NONE),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(32.), Val::Px(32.)),
                            ..default()
                        },
                        image: UiImage(texture_assets.coin.clone()),
                        ..default()
                    });
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(8.), Val::Px(16.)),
                            ..default()
                        },
                        color: UiColor(Color::NONE),
                        ..default()
                    });
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "0".to_string(),
                                    style: TextStyle {
                                        font: font_assets.fira_sans.clone(),
                                        font_size: 40.0,
                                        color: Color::rgb_u8(34, 32, 52),
                                    },
                                }],
                                alignment: Default::default(),
                            },
                            ..Default::default()
                        })
                        .insert(ScoreText);
                });
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        position: Rect {
                            left: Val::Px(5.),
                            top: Val::Px(5.),
                            ..default()
                        },
                        ..default()
                    },
                    color: UiColor(Color::NONE),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(32.), Val::Px(32.)),
                            ..default()
                        },
                        image: UiImage(texture_assets.egg.clone()),
                        ..default()
                    });
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(8.), Val::Px(16.)),
                            ..default()
                        },
                        color: UiColor(Color::NONE),
                        ..default()
                    });
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "0".to_string(),
                                    style: TextStyle {
                                        font: font_assets.fira_sans.clone(),
                                        font_size: 40.0,
                                        color: Color::rgb_u8(34, 32, 52),
                                    },
                                }],
                                alignment: Default::default(),
                            },
                            ..Default::default()
                        })
                        .insert(CurrentEggText);
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "/".to_string(),
                                style: TextStyle {
                                    font: font_assets.fira_sans.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb_u8(34, 32, 52),
                                },
                            }],
                            alignment: Default::default(),
                        },
                        ..Default::default()
                    });
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "1".to_string(),
                                    style: TextStyle {
                                        font: font_assets.fira_sans.clone(),
                                        font_size: 40.0,
                                        color: Color::rgb_u8(34, 32, 52),
                                    },
                                }],
                                alignment: Default::default(),
                            },
                            ..Default::default()
                        })
                        .insert(MaxEggText);
                });
        });
}

fn update_score(mut score_text: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    score_text.single_mut().sections[0].value = format!("{:.0}", score.0.floor());
}
