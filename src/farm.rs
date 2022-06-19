use crate::animal::{Animal, Picked};
use crate::loading::TextureAssets;
use crate::ui::Score;
use crate::{GameState, ANIMAL_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;
use rand::random;

pub struct FarmPlugin;

impl Plugin for FarmPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnTimer>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(spawn)
                .with_system(collect_money),
        );
    }
}

struct SpawnTimer(Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        SpawnTimer(Timer::from_seconds(2., true))
    }
}

fn spawn(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    let animal = Animal::new(time.seconds_since_startup());
    commands
        .spawn_bundle(SpriteBundle {
            texture: animal.generation.get_texture(&textures),
            transform: Transform::from_xyz(
                (random::<f32>() - 0.5) * (WINDOW_WIDTH - ANIMAL_SIZE),
                (random::<f32>() - 0.5) * (WINDOW_HEIGHT - ANIMAL_SIZE),
                0.,
            ),
            ..default()
        })
        .insert(animal);
}

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
