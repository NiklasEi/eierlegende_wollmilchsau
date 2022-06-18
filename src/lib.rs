mod audio;
mod loading;
mod menu;
mod farm;
mod animal;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use crate::animal::AnimalPlugin;
use crate::farm::FarmPlugin;

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
            .add_plugin(InternalAudioPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

fn prepare(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
