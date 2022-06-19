use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_score))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_score));
    }
}

#[derive(Default)]
pub struct Score(pub f32);

#[derive(Component)]
struct ScoreText;

fn spawn_score(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(50.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: Rect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
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
        .with_children(|parent| {
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
}

fn update_score(mut score_text: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    score_text.single_mut().sections[0].value = format!("{:.0}", score.0.round());
}
