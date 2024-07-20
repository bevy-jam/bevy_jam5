use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub(super) fn plugin(_app: &mut App) {}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/ducky.png")]
    #[asset(image(sampler = nearest))]
    pub ducky: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct SfxAssets {
    #[asset(path = "audio/sfx/button_hover.ogg")]
    pub button_hover: Handle<AudioSource>,
    #[asset(path = "audio/sfx/button_press.ogg")]
    pub button_press: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step1.ogg")]
    pub step1: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step2.ogg")]
    pub step2: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step3.ogg")]
    pub step3: Handle<AudioSource>,
    #[asset(path = "audio/sfx/step4.ogg")]
    pub step4: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct SoundtrackAssets {
    #[asset(path = "audio/soundtracks/Monkeys Spinning Monkeys.ogg")]
    pub credits: Handle<AudioSource>,
    #[asset(path = "audio/soundtracks/Fluffing A Duck.ogg")]
    pub gameplay: Handle<AudioSource>,
}
