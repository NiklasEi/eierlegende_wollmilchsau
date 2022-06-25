use crate::animal::{Animal, Picked};
use crate::loading::TextureAssets;
use crate::ui::Score;
use crate::{GameState, ANIMAL_SIZE, UI_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use rand::random;
use std::time::Duration;

pub const BACKGROUND_Z: f32 = 0.;
pub const ANIMAL_Z: f32 = 1.;

pub struct FarmPlugin;

impl Plugin for FarmPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnEggTimer>()
            .init_resource::<CurrentEggs>()
            .init_resource::<CurrentMaxEggs>()
            .init_resource::<CurrentEggTime>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(draw_background))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(spawn)
                    .with_system(collect_money)
                    .with_system(update_spawner_timer),
            );
    }
}

fn draw_background(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(0., 0., BACKGROUND_Z),
        texture: textures.background.clone(),
        ..default()
    });
}

pub struct CurrentEggs(pub u8);

impl Default for CurrentEggs {
    fn default() -> Self {
        CurrentEggs(0)
    }
}

#[derive(Inspectable)]
pub struct CurrentMaxEggs(pub u8);

impl Default for CurrentMaxEggs {
    fn default() -> Self {
        CurrentMaxEggs(1)
    }
}

pub struct CurrentEggTime(pub f32);

impl Default for CurrentEggTime {
    fn default() -> Self {
        CurrentEggTime(10.)
    }
}

pub struct SpawnEggTimer(pub Timer);

impl Default for SpawnEggTimer {
    fn default() -> Self {
        let mut timer = SpawnEggTimer(Timer::from_seconds(10., false));
        timer.0.set_elapsed(Duration::from_secs(9));

        timer
    }
}

fn update_spawner_timer(current_egg_time: Res<CurrentEggTime>, mut timer: ResMut<SpawnEggTimer>) {
    if current_egg_time.is_changed() {
        timer
            .0
            .set_duration(Duration::from_secs_f32(current_egg_time.0));
    }
}

fn spawn(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut timer: ResMut<SpawnEggTimer>,
    mut current_eggs: ResMut<CurrentEggs>,
    current_max_eggs: Res<CurrentMaxEggs>,
    time: Res<Time>,
) {
    if current_max_eggs.0 <= current_eggs.0 {
        return;
    }
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }
    current_eggs.0 += 1;
    timer.0.reset();

    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.egg.clone(),
            transform: Transform::from_xyz(
                (random::<f32>() - 0.5) * (WINDOW_WIDTH - ANIMAL_SIZE - UI_WIDTH) - UI_WIDTH / 2.,
                (random::<f32>() - 0.5) * (WINDOW_HEIGHT - ANIMAL_SIZE),
                ANIMAL_Z,
            ),
            ..default()
        })
        .insert(Egg);
}

#[derive(Component)]
pub struct Egg;

pub fn get_animal_in_reach(
    animals: &Query<(Entity, &Transform, &Animal), Without<Picked>>,
    position: &Vec2,
    reach: f32,
) -> Option<Entity> {
    for (entity, transform, _) in animals.iter() {
        let animal_position = Vec2::new(transform.translation.x, transform.translation.y);
        if animal_position.distance(*position) < reach {
            return Some(entity);
        }
    }

    None
}

fn collect_money(mut score: ResMut<Score>, animals: Query<&Animal>, time: Res<Time>) {
    animals.iter().for_each(|animal| {
        score.0 += animal.generation.money_per_second() * time.delta().as_secs_f32()
    });
}
