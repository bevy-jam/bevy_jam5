//! Spawn the main level by triggering other observers.

use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use bevy::render::mesh::primitives;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::screen::Screen;

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Ground>();
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Name::new("Ground"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(2000.0, 20.0))),
            material: materials.add(Color::srgba(0.5, 0.5, 0.5, 1.)),
            transform: Transform::from_xyz(0.0, 2000.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(2000.0, 20.0),
        Ground,
        StateScoped(Screen::Playing),
    ));

    commands.trigger(SpawnPlayer);
}

#[derive(Default, Debug, Component, Reflect)]
pub struct Ground;
