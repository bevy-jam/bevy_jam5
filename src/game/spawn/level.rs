//! Spawn the main level by triggering other observers.

use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::screen::Screen;

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Ground>();
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        Name::new("Ground"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(1000.))),
            material: materials.add(Color::srgba(0.5,0.5,0.5,1.)),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::circle(1000.0 as Scalar),
        Ground,
        StateScoped(Screen::Playing),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(100.))),
            material: materials.add(Color::srgba(1.,0.5,0.5,1.)),
            transform: Transform::from_xyz(800.0, 0.0, 1.0),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(100.))),
            material: materials.add(Color::srgba(0.5,0.5,1.,1.)),
            transform: Transform::from_xyz(-800.0, 0.0, 1.0),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(100.))),
            material: materials.add(Color::srgba(0.5,1.,0.5,1.)),
            transform: Transform::from_xyz(0.0, 800., 1.0),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(100.))),
            material: materials.add(Color::srgba(1.,1.,1.,1.)),
            transform: Transform::from_xyz(0.0, -800., 1.0),
            ..default()
        },
        StateScoped(Screen::Playing),
    ));

    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnPlayer);
}

#[derive(Default, Debug, Component, Reflect)]
pub struct Ground;
