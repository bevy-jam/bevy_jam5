use bevy::{audio::PlaybackMode, prelude::*};
use rand::seq::SliceRandom;

use crate::game::assets::SfxAssets;

pub(super) fn plugin(app: &mut App) {
    app.observe(play_sfx);
}

fn play_sfx(trigger: Trigger<PlaySfx>, mut commands: Commands, sfx_assets: Res<SfxAssets>) {
    let sfx = match trigger.event() {
        PlaySfx::Handle(handle) => handle.clone_weak(),
        PlaySfx::RandomStep => random_step(&sfx_assets),
    };
    commands.spawn(AudioSourceBundle {
        source: sfx,
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    });
}

/// Trigger this event to play a single sound effect.
#[derive(Event)]
pub enum PlaySfx {
    Handle(Handle<AudioSource>),
    RandomStep,
}

fn random_step(sfx_assets: &SfxAssets) -> Handle<AudioSource> {
    [
        &sfx_assets.step1,
        &sfx_assets.step2,
        &sfx_assets.step3,
        &sfx_assets.step4,
    ]
    .choose(&mut rand::thread_rng())
    .unwrap()
    .clone_weak()
}
