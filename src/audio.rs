use crate::animal::AnimalGeneration;
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioApp, AudioChannel, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimalEvent>()
            .add_plugin(AudioPlugin)
            .add_audio_channel::<Background>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_audio))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(animal_sounds));
    }
}

struct Background;

pub struct AnimalEvent(pub AnimalGeneration);

fn start_audio(audio_assets: Res<AudioAssets>, audio: Res<AudioChannel<Background>>) {
    audio.set_volume(0.2);
    audio.play_looped(audio_assets.background.clone());
}

fn animal_sounds(
    mut events: EventReader<AnimalEvent>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    for event in events.iter() {
        audio.play(event.0.get_audio(&audio_assets));
    }
}
