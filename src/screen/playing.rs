//! The screen state for the main game loop.

use bevy::{
    input::{common_conditions::input_just_pressed, mouse::MouseWheel},
    prelude::*,
};

use super::Screen;
use crate::{
    game::{assets::SoundtrackAssets, audio::soundtrack::PlaySoundtrack, spawn::level::SpawnLevel},
    AppSet,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );

    app.add_systems(
        FixedUpdate,
        zoom_camera
            .after(AppSet::RecordInput)
            .run_if(in_state(Screen::Playing)),
    );
}

fn enter_playing(mut commands: Commands, soundtrack_assets: Res<SoundtrackAssets>) {
    commands.trigger(SpawnLevel);
    // commands.trigger(PlaySoundtrack::Handle(
    //     soundtrack_assets.gameplay.clone_weak(),
    // ));
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct ZoomLevel(f32);

impl Default for ZoomLevel {
    fn default() -> Self {
        Self(0.5)
    }
}

fn zoom_camera(
    mut input: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<Camera>>,
    mut zoom_level: Local<ZoomLevel>,
) {
    for event in input.read() {
        **zoom_level += event.y * 0.16;
    }

    **zoom_level = zoom_level.clamp(0.2, 10.0);

    if let Ok(mut camera) = query.get_single_mut() {
        let delta = Vec3::splat(**zoom_level) - camera.scale;
        camera.scale += delta * 0.03;
    }
}
