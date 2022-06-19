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
    generation: AnimalGeneration,
    state: AnimalState,
}

#[derive(PartialEq)]
pub enum AnimalGeneration {
    Chicken,
    Cow,
    Pig,
}

impl AnimalGeneration {
    fn next(&self) -> Option<Self> {
        match self {
            AnimalGeneration::Chicken => Some(AnimalGeneration::Cow),
            AnimalGeneration::Cow => Some(AnimalGeneration::Pig),
            AnimalGeneration::Pig => None,
        }
    }

    fn get_texture(&self, textures: &TextureAssets) -> Handle<Image> {
        match self {
            AnimalGeneration::Chicken => textures.chicken.clone(),
            AnimalGeneration::Cow => textures.cow.clone(),
            AnimalGeneration::Pig => textures.pig.clone(),
        }
    }
}

pub enum AnimalState {
    Idle,
    Moving { velocity: Vec2 },
    Merging,
}

impl Animal {
    pub(crate) fn random() -> Self {
        Animal {
            generation: AnimalGeneration::Chicken,
            state: AnimalState::Moving {
                velocity: Vec2::new((random::<f32>() * 2.) - 1., (random::<f32>() * 2.) - 1.)
                    .normalize(),
            },
        }
    }
}

fn move_animals(mut animals: Query<(&mut Transform, &Animal), Without<Picked>>) {
    for (mut transform, animal) in animals.iter_mut() {
        if let AnimalState::Moving { velocity } = animal.state {
            transform.translation.x += velocity.x;
            transform.translation.y += velocity.y;
        }
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
    animals: Query<(Entity, &Transform, &Animal), Without<Picked>>,
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
    animals: Query<(Entity, &Transform, &Animal), Without<Picked>>,
    picked_animal: Query<(Entity, &Animal), With<Picked>>,
    windows: Res<Windows>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if !mouse_input.just_released(MouseButton::Left) {
        return;
    }
    if let Ok((picked_animal_entity, picked_animal)) = picked_animal.get_single() {
        if let Some(position) = get_world_coordinates(&windows, &cameras) {
            if let Some(dropped_on_animal) = get_animal_in_reach(&animals, &position, 32.) {
                if picked_animal.generation == animals.get(dropped_on_animal).unwrap().2.generation
                {
                    if let Some(next_generation) = picked_animal.generation.next() {
                        commands.entity(dropped_on_animal).despawn();
                        commands
                            .spawn_bundle(SpriteBundle {
                                texture: next_generation.get_texture(&textures),
                                transform: animals.get(dropped_on_animal).unwrap().1.clone(),
                                ..default()
                            })
                            .insert(Animal {
                                generation: next_generation,
                                state: AnimalState::Idle,
                            });
                        commands.entity(picked_animal_entity).despawn();
                    } else {
                        commands.entity(picked_animal_entity).remove::<Picked>();
                    }
                } else {
                    commands.entity(picked_animal_entity).remove::<Picked>();
                }
            } else {
                commands.entity(picked_animal_entity).remove::<Picked>();
            }
        } else {
            commands.entity(picked_animal_entity).remove::<Picked>();
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
