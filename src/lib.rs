mod animal;
mod audio;
mod farm;
mod loading;
mod menu;
mod ui;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use crate::animal::AnimalPlugin;
use crate::farm::FarmPlugin;
use crate::ui::UiPlugin;
use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

pub const WINDOW_WIDTH: f32 = 800.;
pub const WINDOW_HEIGHT: f32 = 600.;
pub const ANIMAL_SIZE: f32 = 64.;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Menu,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(prepare))
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(FarmPlugin)
            .add_plugin(AnimalPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(UiPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

#[derive(Component)]
pub struct MainCamera;

fn prepare(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}
