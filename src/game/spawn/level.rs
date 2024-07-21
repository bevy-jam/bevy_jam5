//! Spawn the main level by triggering other observers.

use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;

use crate::screen::Screen;

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.spawn((
        Name::new("Ground"),
        SpatialBundle {
            transform: Transform::from_xyz(0.0, -2064.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::circle(2000.0 as Scalar),
        StateScoped(Screen::Playing),
    ));

    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnPlayer);
}
