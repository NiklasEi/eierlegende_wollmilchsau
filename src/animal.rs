use crate::audio::HatchEvent;
use crate::farm::{get_animal_in_reach, CurrentEggs, Egg};
use crate::loading::TextureAssets;
use crate::{GameState, MainCamera, ANIMAL_SIZE, UI_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
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
                .with_system(drop_animal)
                .with_system(update_animal_state),
        );
    }
}

#[derive(Component)]
pub struct Animal {
    pub generation: AnimalGeneration,
    pub state: AnimalState,
}

#[derive(PartialEq)]
pub enum AnimalGeneration {
    Chicken,
    ChickenDuck,
    ChickenDuckGoat,
    ChickenDuckGoatSheep,
    ChickenDuckGoatSheepPig,
    ChickenDuckGoatSheepPigCow,
    ChickenDuckGoatSheepPigCowRabbit,
}

impl AnimalGeneration {
    fn next(&self) -> Option<Self> {
        match self {
            AnimalGeneration::Chicken => Some(AnimalGeneration::ChickenDuck),
            AnimalGeneration::ChickenDuck => Some(AnimalGeneration::ChickenDuckGoat),
            AnimalGeneration::ChickenDuckGoat => Some(AnimalGeneration::ChickenDuckGoatSheep),
            AnimalGeneration::ChickenDuckGoatSheep => {
                Some(AnimalGeneration::ChickenDuckGoatSheepPig)
            }
            AnimalGeneration::ChickenDuckGoatSheepPig => {
                Some(AnimalGeneration::ChickenDuckGoatSheepPigCow)
            }
            AnimalGeneration::ChickenDuckGoatSheepPigCow => {
                Some(AnimalGeneration::ChickenDuckGoatSheepPigCowRabbit)
            }
            AnimalGeneration::ChickenDuckGoatSheepPigCowRabbit => None,
        }
    }

    pub fn get_texture(&self, textures: &TextureAssets) -> Handle<Image> {
        match self {
            AnimalGeneration::Chicken => textures.chicken.clone(),
            AnimalGeneration::ChickenDuck => textures.chicken_2.clone(),
            AnimalGeneration::ChickenDuckGoat => textures.chicken_3.clone(),
            AnimalGeneration::ChickenDuckGoatSheep => textures.chicken_4.clone(),
            AnimalGeneration::ChickenDuckGoatSheepPig => textures.chicken_5.clone(),
            AnimalGeneration::ChickenDuckGoatSheepPigCow => textures.chicken_6.clone(),
            AnimalGeneration::ChickenDuckGoatSheepPigCowRabbit => textures.chicken_7.clone(),
        }
    }

    pub fn money_per_second(&self) -> f32 {
        match self {
            AnimalGeneration::Chicken => 0.5,
            AnimalGeneration::ChickenDuck => 1.5,
            AnimalGeneration::ChickenDuckGoat => 4.,
            AnimalGeneration::ChickenDuckGoatSheep => 9.5,
            AnimalGeneration::ChickenDuckGoatSheepPig => 21.,
            AnimalGeneration::ChickenDuckGoatSheepPigCow => 44.5,
            AnimalGeneration::ChickenDuckGoatSheepPigCowRabbit => 92.,
        }
    }
}

pub enum AnimalState {
    Idle { since: f64 },
    Moving { velocity: Vec2, since: f64 },
}

impl AnimalState {
    fn update(&mut self, seconds_since_startup: f64) {
        match self {
            AnimalState::Idle { .. } => {
                *self = AnimalState::Moving {
                    since: seconds_since_startup,
                    velocity: Vec2::new((random::<f32>() * 2.) - 1., (random::<f32>() * 2.) - 1.)
                        .normalize(),
                }
            }
            AnimalState::Moving { .. } => {
                *self = AnimalState::Idle {
                    since: seconds_since_startup,
                }
            }
        }
    }

    fn change_direction(&mut self, seconds_since_startup: f64) {
        match self {
            AnimalState::Moving {
                ref mut since,
                ref mut velocity,
            } => {
                *since = seconds_since_startup - 1.0;
                *velocity =
                    Vec2::new((random::<f32>() * 2.) - 1., (random::<f32>() * 2.) - 1.).normalize();
            }
            _ => {}
        }
    }

    pub fn can_update_movement(&self, seconds_since_startup: f64) -> bool {
        match self {
            AnimalState::Idle { since } => seconds_since_startup - since > 1.,
            AnimalState::Moving { since, .. } => seconds_since_startup - since > 1.5,
        }
    }
}

impl Animal {
    pub(crate) fn new(seconds_since_startup: f64) -> Self {
        Animal {
            generation: AnimalGeneration::Chicken,
            state: AnimalState::Idle {
                since: seconds_since_startup,
            },
        }
    }
}

fn update_animal_state(mut animals: Query<&mut Animal, Without<Picked>>, time: Res<Time>) {
    for mut animal in animals.iter_mut() {
        if !animal
            .state
            .can_update_movement(time.seconds_since_startup())
        {
            continue;
        }
        let chance = match animal.state {
            AnimalState::Idle { .. } => 0.02,
            AnimalState::Moving { .. } => 0.003,
        };
        if random::<f32>() < chance {
            animal.state.update(time.seconds_since_startup());
        }
    }
}

fn move_animals(
    mut animals: Query<(&mut Transform, &mut Animal), Without<Picked>>,
    time: Res<Time>,
) {
    for (mut transform, mut animal) in animals.iter_mut() {
        if let AnimalState::Moving { velocity, .. } = animal.state {
            transform.translation.x += velocity.x;
            transform.translation.y += velocity.y;

            if transform.translation.x < ANIMAL_SIZE / 2. - WINDOW_WIDTH / 2.
                || transform.translation.x > WINDOW_WIDTH / 2. - ANIMAL_SIZE / 2. - UI_WIDTH
                || transform.translation.y < ANIMAL_SIZE / 2. - WINDOW_HEIGHT / 2.
                || transform.translation.y > WINDOW_HEIGHT / 2. - ANIMAL_SIZE / 2.
            {
                animal.state.change_direction(time.seconds_since_startup());
                transform.translation.x = transform.translation.x.clamp(
                    ANIMAL_SIZE / 2. - WINDOW_WIDTH / 2.,
                    WINDOW_WIDTH / 2. - ANIMAL_SIZE / 2. - UI_WIDTH,
                );
                transform.translation.y = transform.translation.y.clamp(
                    ANIMAL_SIZE / 2. - WINDOW_HEIGHT / 2.,
                    WINDOW_HEIGHT / 2. - ANIMAL_SIZE / 2.,
                );
            }
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

            transform.translation.x = transform.translation.x.clamp(
                ANIMAL_SIZE / 2. - WINDOW_WIDTH / 2.,
                WINDOW_WIDTH / 2. - ANIMAL_SIZE / 2. - UI_WIDTH,
            );
            transform.translation.y = transform.translation.y.clamp(
                ANIMAL_SIZE / 2. - WINDOW_HEIGHT / 2.,
                WINDOW_HEIGHT / 2. - ANIMAL_SIZE / 2.,
            );
        }
    }
}

#[derive(Component)]
pub struct Picked;

fn pick_up_animal(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    time: Res<Time>,
    mut hatch_events: EventWriter<HatchEvent>,
    mut current_eggs: ResMut<CurrentEggs>,
    animals: Query<(Entity, &Transform, &Animal), Without<Picked>>,
    eggs: Query<(Entity, &Transform), (Without<Animal>, With<Egg>)>,
    windows: Res<Windows>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
    if let Some(position) = get_world_coordinates(&windows, &cameras) {
        for (egg, egg_position) in eggs.iter() {
            if position.distance(Vec2::new(
                egg_position.translation.x,
                egg_position.translation.y,
            )) < 32.
            {
                commands.entity(egg).despawn();

                let animal = Animal::new(time.seconds_since_startup());
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: animal.generation.get_texture(&textures),
                        transform: egg_position.clone(),
                        ..default()
                    })
                    .insert(animal);
                current_eggs.0 -= 1;
                hatch_events.send(HatchEvent);
                return;
            }
        }
        if let Some(entity) = get_animal_in_reach(&animals, &position, ANIMAL_SIZE / 2.) {
            commands.entity(entity).insert(Picked);
        }
    }
}

fn drop_animal(
    mut commands: Commands,
    time: Res<Time>,
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
            if let Some(dropped_on_animal) =
                get_animal_in_reach(&animals, &position, ANIMAL_SIZE / 2.)
            {
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
                                state: AnimalState::Idle {
                                    since: time.seconds_since_startup(),
                                },
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
