use crate::{GameState, MainCamera, ShmooLabels};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(process_actions.label(ShmooLabels::ProcessActions)),
        );
    }
}

#[derive(Default)]
pub struct Actions {
    pub just_pressed: bool,
    pub just_released: bool,
    pub position: Option<Vec2>,
}

fn process_actions(
    mut actions: ResMut<Actions>,
    touches_input: Res<Touches>,
    windows: Res<Windows>,
    cameras: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if let Some(touch) = touches_input.iter().next() {
        actions.position = Some(touch.position());
        actions.just_pressed = touches_input.any_just_pressed();
        actions.just_released = touches_input.any_just_released();
        println!("{:?}", touch);
    } else if mouse_input.pressed(MouseButton::Left) || mouse_input.just_released(MouseButton::Left)
    {
        if let Some(position) = get_world_coordinates(&windows, &cameras) {
            actions.position = Some(position);
            actions.just_pressed = mouse_input.just_pressed(MouseButton::Left);
            actions.just_released = mouse_input.just_released(MouseButton::Left);
        }
    } else {
        actions.position = None;
        actions.just_pressed = false;
        actions.just_released = false;
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
