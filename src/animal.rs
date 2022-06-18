use crate::farm::get_animal_in_reach;
use crate::loading::TextureAssets;
use crate::{GameState, MainCamera};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use rand::random;

pub struct AnimalPlugin;

impl Plugin for AnimalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_animals)
                .with_system(pick_up_animal)
                .with_system(move_picked_animal)
                .with_system(drop_animal),
        );
    }
}

#[derive(Component)]
pub struct Animal {
    velocity: Vec2,
}

impl Animal {
    pub(crate) fn random() -> Self {
        Animal {
            velocity: Vec2::new((random::<f32>() * 2.) - 1., (random::<f32>() * 2.) - 1.),
        }
    }
}

fn move_animals(mut animals: Query<(&mut Transform, &Animal), Without<Picked>>) {
    for (mut transform, animal) in animals.iter_mut() {
        transform.translation.x += animal.velocity.x;
        transform.translation.y += animal.velocity.y;
    }
}

fn move_picked_animal(
    mut animal: Query<&mut Transform, With<Picked>>,
    windows: Res<Windows>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if let Ok(mut transform) = animal.get_single_mut() {
        if let Some(position) = get_world_coordinates(&windows, &cameras) {
            transform.translation.x = position.x;
            transform.translation.y = position.y;
        }
    }
}

#[derive(Component)]
pub struct Picked;

fn pick_up_animal(
    mut commands: Commands,
    animals: Query<(Entity, &Transform), (With<Animal>, Without<Picked>)>,
    windows: Res<Windows>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
    if let Some(position) = get_world_coordinates(&windows, &cameras) {
        eprintln!("World coords: {}/{}", position.x, position.y);
        if let Some(entity) = get_animal_in_reach(&animals, &position, 32.) {
            commands.entity(entity).insert(Picked);
        }
    }
}

fn drop_animal(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    mouse_input: Res<Input<MouseButton>>,
    animals: Query<(Entity, &Transform), (With<Animal>, Without<Picked>)>,
    picked_animals: Query<Entity, With<Picked>>,
    windows: Res<Windows>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if !mouse_input.just_released(MouseButton::Left) {
        return;
    }
    if let Some(position) = get_world_coordinates(&windows, &cameras) {
        if let Some(entity) = get_animal_in_reach(&animals, &position, 32.) {
            commands.entity(entity).despawn();
            commands
                .spawn_bundle(SpriteBundle {
                    texture: textures.cow.clone(),
                    transform: animals.get(entity).unwrap().1.clone(),
                    ..default()
                })
                .insert(Animal::random());
            for entity in picked_animals.iter() {
                commands.entity(entity).despawn();
            }
        } else {
            for entity in picked_animals.iter() {
                commands.entity(entity).remove::<Picked>();
            }
        }
    } else {
        for entity in picked_animals.iter() {
            commands.entity(entity).remove::<Picked>();
        }
    }
}

// See https://bevy-cheatbook.github.io/cookbook/cursor2world.html
fn get_world_coordinates(
    windows: &Res<Windows>,
    cameras: &Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = cameras.single();
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };
    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        return Some(world_pos.truncate());
    }

    None
}
