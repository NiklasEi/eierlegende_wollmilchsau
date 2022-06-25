use crate::animal::{Animal, AnimalGeneration};
use crate::farm::{CurrentEggTime, CurrentEggs, CurrentMaxEggs};
use crate::loading::{FontAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use strum::IntoEnumIterator;

const UI_WIDTH: f32 = 180.;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .init_resource::<Score>()
            .insert_resource(MaxEggPrice(1000.))
            .insert_resource(EggTimePrice(10.))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_ui))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_score)
                    .with_system(update_current_eggs)
                    .with_system(update_current_max_eggs)
                    .with_system(update_question_marks)
                    .with_system(buy_max_egg)
                    .with_system(buy_faster_eggs)
                    .with_system(update_egg_time),
            );
    }
}

#[derive(Default, Inspectable)]
pub struct Score(pub f32);

#[derive(Component)]
struct ScoreText;
#[derive(Component)]
struct CurrentEggText;
#[derive(Component)]
struct CurrentEggTimerText;
#[derive(Component)]
struct MaxEggText;
#[derive(Component)]
struct DecreaseEggTimeButton;
#[derive(Component)]
struct IncreaseMaxEggsButton;

fn spawn_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    texture_assets: Res<TextureAssets>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(UI_WIDTH), Val::Percent(100.)),
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
            color: UiColor(Color::NONE),
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
        // Egg timer
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
                        image: UiImage(texture_assets.egg_timer.clone()),
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
                                    value: "10".to_string(),
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
                        .insert(CurrentEggTimerText);
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                margin: Rect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                position: Rect {
                                    left: Val::Px(10.),
                                    ..default()
                                },
                                ..Default::default()
                            },
                            color: button_colors.normal,
                            ..Default::default()
                        })
                        .insert(DecreaseEggTimeButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "-".to_string(),
                                        style: TextStyle {
                                            font: font_assets.fira_sans.clone(),
                                            font_size: 40.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    }],
                                    alignment: Default::default(),
                                },
                                ..Default::default()
                            });
                        });
                });
        })
        // Current max eggs
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
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                margin: Rect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                position: Rect {
                                    left: Val::Px(10.),
                                    ..default()
                                },
                                ..Default::default()
                            },
                            color: button_colors.normal,
                            ..Default::default()
                        })
                        .insert(IncreaseMaxEggsButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "+".to_string(),
                                        style: TextStyle {
                                            font: font_assets.fira_sans.clone(),
                                            font_size: 40.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    }],
                                    alignment: Default::default(),
                                },
                                ..Default::default()
                            });
                        });
                });
        })
        // Explain text
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Px(32.)),
                        ..default()
                    },
                    color: UiColor(Color::NONE),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "".to_string(),
                                    style: TextStyle {
                                        font: font_assets.fira_sans.clone(),
                                        font_size: 15.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                }],
                                alignment: Default::default(),
                            },
                            ..Default::default()
                        })
                        .insert(ExplainText);
                });
        })
        // Animal collection
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::Center,
                        flex_wrap: FlexWrap::WrapReverse,

                        position: Rect {
                            left: Val::Px(5.),
                            top: Val::Px(15.),
                            ..default()
                        },
                        ..default()
                    },
                    color: UiColor(Color::NONE),
                    ..default()
                })
                .with_children(|parent| {
                    for animal in AnimalGeneration::iter() {
                        parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(64.), Val::Px(64.)),
                                    ..default()
                                },
                                image: UiImage(texture_assets.question_mark.clone()),
                                ..default()
                            })
                            .insert(UiAnimal(animal))
                            .insert(QuestionMark);
                    }
                });
        });
}

#[derive(Component)]
struct ExplainText;

#[derive(Component)]
struct UiAnimal(AnimalGeneration);

#[derive(Component)]
struct QuestionMark;

fn update_question_marks(
    mut question_marks: Query<(&mut UiImage, &UiAnimal), (With<QuestionMark>, Without<Animal>)>,
    new_animal: Query<&Animal, Added<Animal>>,
    textures: Res<TextureAssets>,
) {
    for (mut image, ui_animal) in question_marks.iter_mut() {
        let new_animal = new_animal
            .iter()
            .find(|animal| animal.generation == ui_animal.0);
        if new_animal.is_some() {
            image.0 = ui_animal.0.get_texture(&textures);
        }
    }
}

fn update_score(mut score_text: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        score_text.single_mut().sections[0].value = format!("{:.0}", score.0.floor());
    }
}

fn update_current_eggs(
    mut egg_text: Query<&mut Text, With<CurrentEggText>>,
    current_eggs: Res<CurrentEggs>,
) {
    if current_eggs.is_changed() {
        egg_text.single_mut().sections[0].value = format!("{}", current_eggs.0);
    }
}

fn update_current_max_eggs(
    mut egg_text: Query<&mut Text, With<MaxEggText>>,
    current_max_eggs: Res<CurrentMaxEggs>,
) {
    if current_max_eggs.is_changed() {
        egg_text.single_mut().sections[0].value = format!("{}", current_max_eggs.0);
    }
}

pub struct MaxEggPrice(pub f32);

fn buy_max_egg(
    button_colors: Res<ButtonColors>,
    mut current_max_eggs: ResMut<CurrentMaxEggs>,
    mut max_egg_price: ResMut<MaxEggPrice>,
    mut score: ResMut<Score>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (
            Changed<Interaction>,
            With<IncreaseMaxEggsButton>,
            Without<ExplainText>,
        ),
    >,
    mut explain_text: Query<&mut Text, With<ExplainText>>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if score.0 > max_egg_price.0 {
                    score.0 -= max_egg_price.0;
                    current_max_eggs.0 += 1;
                    max_egg_price.0 *= 10.;
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered;
                let mut text = explain_text.single_mut();
                text.sections.get_mut(0).unwrap().value =
                    format!("+1 max eggs for {} G", max_egg_price.0);
            }
            Interaction::None => {
                *color = button_colors.normal;
                let mut text = explain_text.single_mut();
                text.sections.get_mut(0).unwrap().value = "".to_owned();
            }
        }
    }
}

pub struct EggTimePrice(pub f32);

fn buy_faster_eggs(
    button_colors: Res<ButtonColors>,
    mut current_egg_time: ResMut<CurrentEggTime>,
    mut egg_time_price: ResMut<EggTimePrice>,
    mut score: ResMut<Score>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (
            Changed<Interaction>,
            With<DecreaseEggTimeButton>,
            Without<ExplainText>,
        ),
    >,
    mut explain_text: Query<&mut Text, With<ExplainText>>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if score.0 > egg_time_price.0 && current_egg_time.0 > 1.5 {
                    score.0 -= egg_time_price.0;
                    current_egg_time.0 -= 1.;
                    egg_time_price.0 *= 10.;
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered;
                let mut text = explain_text.single_mut();
                text.sections.get_mut(0).unwrap().value =
                    format!("-1s for egg: {} G", egg_time_price.0);
            }
            Interaction::None => {
                *color = button_colors.normal;
                let mut text = explain_text.single_mut();
                text.sections.get_mut(0).unwrap().value = "".to_owned();
            }
        }
    }
}

fn update_egg_time(
    current_egg_time: Res<CurrentEggTime>,
    mut egg_time_text: Query<&mut Text, With<CurrentEggTimerText>>,
) {
    if current_egg_time.is_changed() {
        egg_time_text
            .single_mut()
            .sections
            .get_mut(0)
            .unwrap()
            .value = format!("{:.0}", current_egg_time.0.floor());
    }
}

pub struct ButtonColors {
    pub normal: UiColor,
    pub hovered: UiColor,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
        }
    }
}
