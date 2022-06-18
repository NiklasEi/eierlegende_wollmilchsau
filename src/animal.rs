use bevy::prelude::*;
use rand::random;
use crate::GameState;

pub struct AnimalPlugin;

impl Plugin for AnimalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_animals));
    }
}

#[derive(Component)]
pub struct Animal {
    velocity: Vec2
}

impl Animal {
    pub(crate) fn random() -> Self {
        Animal {
            velocity: Vec2::new((random::<f32>() * 2.) - 1., (random::<f32>() * 2.) - 1.)
        }
    }
}

fn move_animals(mut animals: Query<(&mut Transform, &Animal)>) {
    for (mut transform, animal) in animals.iter_mut() {
        transform.translation.x += animal.velocity.x;
        transform.translation.y += animal.velocity.y;
    }
}
