use crate::animal::{Animal, Picked};
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct FarmPlugin;

impl Plugin for FarmPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnTimer>()
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn));
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

    commands
        .spawn_bundle(SpriteBundle {
            texture: textures.chicken.clone(),
            ..default()
        })
        .insert(Animal::random());
}

pub fn get_animal_in_reach(
    animals: &Query<(Entity, &Transform), (With<Animal>, Without<Picked>)>,
    position: &Vec2,
    reach: f32,
) -> Option<Entity> {
    for (entity, transform) in animals.iter() {
        let animal_position = Vec2::new(transform.translation.x, transform.translation.y);
        if animal_position.distance(*position) < reach {
            return Some(entity);
        }
    }

    None
}
