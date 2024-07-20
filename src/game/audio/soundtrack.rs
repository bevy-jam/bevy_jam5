use bevy::{audio::PlaybackMode, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IsSoundtrack>();
    app.observe(play_soundtrack);
}

fn play_soundtrack(
    trigger: Trigger<PlaySoundtrack>,
    mut commands: Commands,
    soundtrack_query: Query<Entity, With<IsSoundtrack>>,
) {
    for entity in &soundtrack_query {
        commands.entity(entity).despawn_recursive();
    }

    let soundtrack_handle = match trigger.event() {
        PlaySoundtrack::Handle(handle) => handle.clone_weak(),
        PlaySoundtrack::Disable => return,
    };
    commands.spawn((
        AudioSourceBundle {
            source: soundtrack_handle,
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        IsSoundtrack,
    ));
}

/// Trigger this event to play or disable the soundtrack.
/// Playing a new soundtrack will overwrite the previous one.
/// Soundtracks will loop.
#[derive(Event)]
pub enum PlaySoundtrack {
    Handle(Handle<AudioSource>),
    Disable,
}

/// Marker component for the soundtrack entity so we can find it later.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsSoundtrack;
