//! Spawn the main level by triggering other observers.

use avian2d::math::Vector;
use avian2d::{math::Scalar, prelude::*};
use bevy::math::vec2;
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
    let material = materials.add(Color::srgba(0.5, 0.5, 0.5, 1.));

    commands.spawn((
        Name::new("Ground"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(4000.0, 20.0))),
            material: material.clone(),
            transform: Transform::from_xyz(0.0, 20000.0, 0.0),
            ..default()
        },
        Collider::rectangle(4000.0, 20.0),
        RigidBody::Static,
        Ground,
        StateScoped(Screen::Playing),
    ));

    commands.spawn((
        Name::new("Platform"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(400.0, 20.0))),
            material: material.clone(),
            transform: Transform::from_xyz(300.0, 20100.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(400.0, 20.0),
        Ground,
        StateScoped(Screen::Playing),
    ));

    commands.spawn((
        Name::new("Platform"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(400.0, 20.0))),
            material: material.clone(),
            transform: Transform::from_xyz(700.0, 20090.0, 0.0)
                .with_rotation(Quat::from_rotation_z(f32::to_radians(30.0))),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(400.0, 20.0),
        Ground,
        StateScoped(Screen::Playing),
    ));

    commands.spawn((
        Name::new("Platform"),
        Transform::from_xyz(-100.0, 20070.0, 0.0)
            .with_rotation(Quat::from_rotation_z(f32::to_radians(-14.0))),
        RigidBody::Static,
        Collider::polyline(
            vec![
                vec2(0.0, 0.0),
                vec2(0.0, 70.0),
                vec2(60.0, 58.0),
                vec2(120.0, 70.0),
                vec2(120.0, 0.0),
            ],
            None,
        ),
        Ground,
        StateScoped(Screen::Playing),
    ));

    commands.spawn((
        Name::new("Platform"),
        Transform::from_xyz(-1000.0, 19850.0, 0.0),
        RigidBody::Static,
        Collider::ellipse(600.0, 300.0),
        Ground,
        StateScoped(Screen::Playing),
    ));

    let vertices = vec![
        vec2(0.0, 0.0),
        vec2(10.0, 7.0),
        vec2(15.0, 10.0),
        vec2(20.0, 8.0),
        vec2(27.0, 17.0),
        vec2(30.0, 3.0),
        vec2(34.0, 16.0),
        vec2(40.0, 14.0),
        vec2(47.0, 0.0),
        vec2(50.0, 34.0),
        vec2(56.0, 25.0),
        vec2(60.0, 42.0),
        vec2(110.0, 14.0),
        vec2(140.0, 0.0),
    ];

    let mut indices: Vec<_> = (0..vertices.len())
        .map(|i| [i as u32, i as u32 + 1])
        .collect();

    indices.last_mut().unwrap()[1] = 0;

    commands.spawn((
        Name::new("Platform"),
        Transform::from_xyz(1000.0, 20012.0, 0.0).with_scale(Vec3::splat(5.0)),
        RigidBody::Static,
        Collider::convex_decomposition_with_config(
            vertices,
            indices,
            &VhacdParameters {
                concavity: 0.012,
                resolution: 300,
                plane_downsampling: 4,
                ..VhacdParameters::default()
            },
        ),
        Ground,
        StateScoped(Screen::Playing),
    ));

    commands.trigger(SpawnPlayer);
}

#[derive(Default, Debug, Component, Reflect, Clone)]
pub struct Ground;
