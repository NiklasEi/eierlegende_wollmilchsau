use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_dynamic_asset_collection_file("dynamic.assets")
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .continue_to_state(GameState::Menu)
            .build(app);
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/background.ogg")]
    pub background: Handle<AudioSource>,
    #[asset(path = "audio/chicken_hatch.ogg")]
    pub chicken_hatch: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(key = "background")]
    pub background: Handle<Image>,
    #[asset(key = "coin")]
    pub coin: Handle<Image>,
    #[asset(key = "egg")]
    pub egg: Handle<Image>,
    #[asset(key = "chicken")]
    pub chicken: Handle<Image>,
    #[asset(key = "chicken_2")]
    pub chicken_2: Handle<Image>,
    #[asset(key = "chicken_3")]
    pub chicken_3: Handle<Image>,
    #[asset(key = "chicken_4")]
    pub chicken_4: Handle<Image>,
    #[asset(key = "chicken_5")]
    pub chicken_5: Handle<Image>,
    #[asset(key = "chicken_6")]
    pub chicken_6: Handle<Image>,
    #[asset(key = "chicken_7")]
    pub chicken_7: Handle<Image>,
}
