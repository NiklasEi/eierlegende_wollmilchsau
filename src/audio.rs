use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HatchEvent>()
            .add_plugin(AudioPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_audio))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(chicken_hatch));
    }
}

pub struct HatchEvent;

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.set_volume(0.3);
    audio.play_looped(audio_assets.background.clone());
}

fn chicken_hatch(
    mut events: EventReader<HatchEvent>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    for _event in events.iter() {
        audio.play(audio_assets.chicken_hatch.clone());
    }
}
